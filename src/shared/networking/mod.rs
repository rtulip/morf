mod tcp;

pub use tcp::{NetworkEvent, NetworkResource, TcpSystemBundle};

pub const TCP_CONNECTION_LISTENER_SYSTEM_NAME: &'static str = "TcpConnectionListenerSystem";
pub const TCP_NETWORK_EVENT_HANDLER_SYSTEM_NAME: &'static str = "TcpNetworkEventHandlerSystem";
pub const TCP_NETWORK_LISTENER_SYSTEM_NAME: &'static str = "TcpNetworkListenerSystem";
