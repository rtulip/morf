use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, Read, System, World, Write},
    shrev::EventChannel,
    Error,
};
use bytes::Bytes;
use log::*;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

use std::{io, net::SocketAddr};

#[derive(Debug)]
pub enum NetworkEvent {
    _Message(SocketAddr, Bytes),
    Connect(SocketAddr),
    _Disconnect(SocketAddr),
    _ConnectionError(io::Error, Option<SocketAddr>),
}

#[derive(Default)]
pub struct NetworkResource {
    listener: Option<TcpListener>,
    _streams: HashMap<SocketAddr, TcpStream>,
}

impl NetworkResource {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener: Some(listener),
            _streams: HashMap::default(),
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
        builder.add(TcpListenerSystem, "TcpListenerSystem", &[]);
        world.insert(NetworkResource::new(self.listener));
        if let Some(addr) = self.server_addr {
            if let Ok(_stream) = TcpStream::connect(addr) {
                info!("Connected to server!");
            } else {
                warn!("Failed to connect to server!");
            }
        }

        Ok(())
    }
}

pub struct TcpListenerSystem;

impl<'a> System<'a> for TcpListenerSystem {
    type SystemData = (
        Read<'a, NetworkResource>,
        Write<'a, EventChannel<NetworkEvent>>,
    );

    fn run(&mut self, (network_res, mut channel): Self::SystemData) {
        loop {
            if let Some(ref listener) = network_res.listener {
                match listener.accept() {
                    Ok((_stream, addr)) => {
                        info!("New connection: {}", addr);
                        channel.single_write(NetworkEvent::Connect(addr));
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
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
