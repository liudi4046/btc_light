use super::factory;
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
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized_netaddr: Vec<u8> = Vec::new();
        serialized_netaddr.extend(&self.services.to_le_bytes());
        serialized_netaddr.extend(&self.ip_address.octets());
        serialized_netaddr.extend(&self.port.to_le_bytes());
        serialized_netaddr
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
    user_agent: u8,
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
        user_agent: u8,
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
    //with_addr_recv用于更改当前versionMessage的消息接受者的ip地址
    pub fn with_addr_recv(mut self, ipv4_recv: Ipv4Addr) -> Self {
        let addr_recv = NetAddr::new(0, ipv4_recv.to_ipv6_mapped(), 8333);

        self.addr_recv = addr_recv;
        self
    }
}
impl Default for VersionMessage {
    //defualt方法用于生成一个默认的VersionMessage，以便后续更新其他字段比如接收消息的ip。
    fn default() -> Self {
        let time_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = time_since_epoch.as_secs().try_into().unwrap();
        let public_ip_from: Ipv4Addr = reqwest::get("http://api.ipify.org")
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
            user_agent: 0,
            start_height: 0,
        }
    }
}
impl Message for VersionMessage {
    fn serialize(&self) -> Vec<u8> {
        let mut serialized_message: Vec<u8> = Vec::new();
        serialized_message.extend(&self.version.to_le_bytes());
        serialized_message.extend(&self.services.to_le_bytes());
        serialized_message.extend(&self.timestamp.to_le_bytes());

        serialized_message.extend(self.addr_recv.serialize());
        serialized_message.extend(self.addr_from.serialize());

        serialized_message.extend(&self.nonce.to_le_bytes());
        serialized_message.extend(&self.user_agent.to_le_bytes());
        serialized_message.extend(&self.start_height.to_le_bytes());

        serialized_message
    }
    //每个消息payload都有自己创建消息头的方法，消息头字段：magic,command,payload size, checksum
    // fn create_header(&self) -> super::factory::MessageHeader {
    //     let magic = 0xD9B4BEF9;
    //     let command = [0u8; 12];
    //     command[..command.len()].copy_from_slice("version".as_bytes());

    //     let serialized_payload = self.serialize();
    //     let length = serialized_payload.len() as u32;
    //     let checksum = factory::checksum(&serialized_payload);

    //     factory::MessageHeader::new(0xD9B4BEF9, &command, length, checksum)
    // }
}
