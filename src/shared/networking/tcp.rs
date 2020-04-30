use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    shrev::{EventChannel, ReaderId},
    Error,
};
use bytes::Bytes;
use log::*;
use std::collections::HashMap;
use std::io::{Error as IoError, ErrorKind, Read as IoRead, Result as IoResult, Write as IoWrite};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::ops::DerefMut;

const TCP_CONNECTION_LISTENER_SYSTEM_NAME: &'static str = "TcpConnectionListenerSystem";
const TCP_NETWORK_EVENT_HANDLER_SYSTEM_NAME: &'static str = "TcpNetworkEventHandlerSystem";
const TCP_NETWORK_LISTENER_SYSTEM_NAME: &'static str = "TcpNetworkListenerSystem";

#[derive(Debug)]
pub enum NetworkEvent {
    Message(SocketAddr, Bytes),
    Connect(SocketAddr),
    _Disconnect(SocketAddr),
    _ConnectionError(IoError, Option<SocketAddr>),
}

#[derive(Default)]
pub struct NetworkResource {
    listener: Option<TcpListener>,
    streams: HashMap<SocketAddr, TcpStream>,
}

impl NetworkResource {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener: Some(listener),
            streams: HashMap::default(),
        }
    }

    pub fn send(&mut self, dest: &SocketAddr, msg: &'_ str) -> IoResult<usize> {
        if let Some(stream) = self.streams.get_mut(&dest) {
            let n_bytes = stream.write(msg.as_bytes())?;
            Ok(n_bytes)
        } else {
            Err(IoError::new(
                ErrorKind::NotConnected,
                "Not Connected to stream",
            ))
        }
    }

    pub fn set_listener(&mut self, listener: TcpListener) {
        self.listener = Some(listener);
    }
}

pub struct TcpSystemBundle {
    listener: TcpListener,
    server_addr: Option<SocketAddr>,
}

impl TcpSystemBundle {
    pub fn new(listener: TcpListener, server_addr: Option<SocketAddr>) -> Self {
        Self {
            listener,
            server_addr,
        }
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for TcpSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'_, '_>,
    ) -> Result<(), Error> {
        builder.add(
            TcpConnectionListenerSystem,
            TCP_CONNECTION_LISTENER_SYSTEM_NAME,
            &[],
        );
        builder.add(
            TcpNetworkEventHandlerSystem::new(world),
            TCP_NETWORK_EVENT_HANDLER_SYSTEM_NAME,
            &[TCP_CONNECTION_LISTENER_SYSTEM_NAME],
        );
        builder.add(
            TcpNetworkListenerSystem,
            TCP_NETWORK_LISTENER_SYSTEM_NAME,
            &[
                TCP_CONNECTION_LISTENER_SYSTEM_NAME,
                TCP_NETWORK_EVENT_HANDLER_SYSTEM_NAME,
            ],
        );
        let mut network_res = NetworkResource::new(self.listener);
        if let Some(addr) = self.server_addr {
            if let Ok(stream) = TcpStream::connect(addr) {
                info!("Connected to server!");
                network_res.streams.insert(addr, stream);
            } else {
                warn!("Failed to connect to server!");
            }
        }

        world.insert(network_res);
        Ok(())
    }
}

pub struct TcpConnectionListenerSystem;

impl<'a> System<'a> for TcpConnectionListenerSystem {
    type SystemData = (
        Write<'a, NetworkResource>,
        Write<'a, EventChannel<NetworkEvent>>,
    );

    fn run(&mut self, (mut network_res, mut channel): Self::SystemData) {
        loop {
            if let Some(ref mut listener) = network_res.listener {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        info!("New connection: {}", addr);
                        stream.set_nonblocking(true).expect("Setting nonblocking");
                        stream.set_nodelay(true).expect("Setting nodelay");
                        network_res.streams.insert(addr, stream);
                        channel.single_write(NetworkEvent::Connect(addr));
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(e) => {
                        error!("Network Error: {}", e);
                    }
                }
            } else {
                warn!("No NetworkListener");
            }
        }
    }
}

pub struct TcpNetworkListenerSystem;

impl<'a> System<'a> for TcpNetworkListenerSystem {
    type SystemData = (
        Write<'a, EventChannel<NetworkEvent>>,
        Write<'a, NetworkResource>,
    );
    fn run(&mut self, (mut channel, mut resource): Self::SystemData) {
        let res = resource.deref_mut();
        for (addr, stream) in res.streams.iter_mut() {
            let mut buff = [0; 128];
            match stream.read(&mut buff) {
                Ok(n_bytes) => {
                    if n_bytes > 0 {
                        info!("Read {} bytes from {}", n_bytes, addr);
                        channel.single_write(NetworkEvent::Message(
                            *addr,
                            Bytes::copy_from_slice(&buff[..n_bytes]),
                        ));
                    }
                }
                Err(e ) if e.kind() == ErrorKind::WouldBlock => {},
                Err(e) => {
                    error!("Network Recv Error: {}", e);
                }
            }
        }
    }
}

pub struct TcpNetworkEventHandlerSystem {
    reader_id: ReaderId<NetworkEvent>,
}

impl TcpNetworkEventHandlerSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world
            .fetch_mut::<EventChannel<NetworkEvent>>()
            .register_reader();
        Self { reader_id }
    }
}

impl<'a> System<'a> for TcpNetworkEventHandlerSystem {
    type SystemData = (
        Read<'a, EventChannel<NetworkEvent>>,
        Write<'a, NetworkResource>,
    );
    fn run(&mut self, (channel, mut resource): Self::SystemData) {
        for event in channel.read(&mut self.reader_id) {
            match event {
                NetworkEvent::Connect(addr) => {
                    match resource.send(addr, "Thanks for joining the Server") {
                        Ok(n_bytes) => info!("Sent {} bytes", n_bytes),
                        Err(e) => error!("Network Send Error: {}", e),
                    }
                },
                NetworkEvent::Message(addr, bytes) => {
                    info!("Recieved {:?} from {}", bytes, addr);
                }
                _ => {}
            }
        }
    }
}
