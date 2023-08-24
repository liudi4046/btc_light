use super::factory::Message;
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct NetAddr {
    services: u64,
    ip_address: Ipv6Addr,
    port: u16,
}

impl NetAddr {
    pub fn new(services: u64, ip_address: Ipv6Addr, port: u16) -> Self {
        NetAddr {
            services,
            ip_address,
            port,
        }
    }
}

#[derive(Debug)]
pub struct VersionMessage {
    version: i32,
    services: u64,
    timestamp: i64,
    addr_recv: NetAddr,
    addr_from: NetAddr,
    nonce: u64,
    user_agent: String,
    start_height: i32,
}

impl VersionMessage {
    pub fn new(
        version: i32,
        services: u64,
        timestamp: i64,
        addr_recv: NetAddr,
        addr_from: NetAddr,
        nonce: u64,
        user_agent: String,
        start_height: i32,
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
        }
    }
    pub fn with_addr_recv(mut self, ipv4_recv: Ipv4Addr) -> Self {
        let addr_recv = NetAddr::new(0, ipv4_recv.to_ipv6_mapped(), 8333);

        self.addr_recv = addr_recv;
        self
    }
}
impl Default for VersionMessage {
    fn default() -> Self {
        let time_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = time_since_epoch.as_secs().try_into().unwrap();
        let public_ip_from: Ipv4Addr = reqwest::blocking::get("http://api.ipify.org")
            .unwrap()
            .text()
            .unwrap()
            .parse()
            .unwrap();
        let ipv6_from = public_ip_from.to_ipv6_mapped();

        VersionMessage {
            version: 70015, // this is an example, you might want to set your own default
            services: 0,
            timestamp,
            addr_recv: NetAddr::new(0, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8333),
            addr_from: NetAddr::new(0, ipv6_from, 8333),
            nonce: 0,
            user_agent: "/blue_blink:0.0.1/".to_string(),
            start_height: 0,
        }
    }
}
impl Message for VersionMessage {
    fn serialize(&self) -> Vec<u8> {}
    fn deserialize(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
    }
}
