use amethyst::{
    ecs::{System, Write},
    shrev::EventChannel,
};
use bytes::Bytes;
use log::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct TcpListenerBundle;

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
    stream: Option<TcpStream>,
}

pub struct TcpListenerSystem {
    listener: TcpListener,
}

impl TcpListenerSystem {
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }
}

impl<'a> System<'a> for TcpListenerSystem {
    type SystemData = Write<'a, EventChannel<NetworkEvent>>;

    fn run(&mut self, mut channel: Self::SystemData) {
        loop {
            match self.listener.accept() {
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
        }
    }
}

pub struct TcpConnectorSystem;

impl<'a> System<'a> for TcpConnectorSystem {
    type SystemData = Write<'a, NetworkResource>;

    fn run(&mut self, mut manager: Self::SystemData) {
        if let Some(_stream) = &manager.stream {
            // Do nothing
        } else {
            if let Ok(stream) = TcpStream::connect("localhost:8080") {
                info!("Connected to server");
                manager.stream = Some(stream);
            } else {
                error!("Failed to connect to server");
            }
        }
    }
}
