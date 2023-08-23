use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct VersionMessage {
    version: i32,
    services: u64,
    timestamp: i64,
    addr_recv: SocketAddr,
    addr_from: SocketAddr,
    nonce: u64,
    user_agent: String,
    start_height: i32,
    relay: Option<bool>, // Optional depending on version number
}

impl VersionMessage {
    pub fn new(
        version: i32,
        services: u64,
        timestamp: i64,
        addr_recv: SocketAddr,
        addr_from: SocketAddr,
        nonce: u64,
        user_agent: String,
        start_height: i32,
        relay: Option<bool>,
    ) -> Self {
        VersionMessage {
            version,
            services,
            timestamp,
            addr_recv,
            addr_from,
            nonce,
            user_agent,
            start_height,
            relay,
        }
    }

    pub fn with_defaults() -> Self {
        Self::default()
    }
}
impl Default for VersionMessage {
    fn default() -> Self {
        VersionMessage {
            version: 70015, // this is an example, you might want to set your own default
            services: 0,
            timestamp: 0,
            addr_recv: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8333),
            addr_from: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8333),
            nonce: 0,
            user_agent: "/your-client-name:0.0.1/".to_string(),
            start_height: 0,
            relay: Some(true),
        }
    }
}
