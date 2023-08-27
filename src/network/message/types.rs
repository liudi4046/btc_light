use super::factory::Message;
use byteorder::BigEndian;
#[derive(Debug)]
use byteorder::{LittleEndian, ReadBytesExt};
use rand::Rng;
use std::{
    io::Read,
    net::{Ipv4Addr, Ipv6Addr},
    time::{SystemTime, UNIX_EPOCH},
};
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
        serialized_netaddr.extend(&self.port.to_be_bytes());
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
    start_height: u32,
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
        start_height: u32,
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
    pub fn with_addr_from(mut self, ipv4_from: Ipv4Addr) -> Self {
        let addr_from = NetAddr::new(0, ipv4_from.to_ipv6_mapped(), 8333);
        self.addr_from = addr_from;
        self
    }
}
impl Default for VersionMessage {
    //defualt方法用于生成一个默认的VersionMessage，以便后续更新其他字段比如接收消息的ip和本机的公共ip。
    fn default() -> Self {
        let time_since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = time_since_epoch.as_secs().try_into().unwrap();
        let mut rng = rand::thread_rng();
        let random_value: u64 = rng.gen();
        VersionMessage {
            version: 70015,
            services: 0,
            timestamp,
            addr_recv: NetAddr::new(0, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8333),
            addr_from: NetAddr::new(0, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8333),
            nonce: random_value,
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
    fn deserialize(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        //head
        let magic = cursor.read_u32::<LittleEndian>()?;

        let mut command: [u8; 12] = [0; 12];
        cursor.read_exact(&mut command);

        let length = cursor.read_u32::<LittleEndian>()?;

        let checksum: [u8; 4] = [0; 4];
        cursor.read_exact(&mut checksum);

        //payload
        let version = cursor.read_i32::<LittleEndian>()?;
        let services = cursor.read_u64::<LittleEndian>()?;
        let timestamp = cursor.read_i64::<LittleEndian>()?;

        // addr_recv
        let addr_recv_services = cursor.read_u64::<LittleEndian>()?;
        let mut addr_recv_ip_bytes: [u8; 16] = [0; 16];
        cursor.read_exact(&mut addr_recv_ip_bytes);
        let addr_recv_ip = Ipv6Addr::from(addr_recv_ip_bytes);
        let addr_recv_port = cursor.read_u16::<BigEndian>()?;

        //addr_trans
        let addr_trans_services = cursor.read_u64::<LittleEndian>()?;
        let mut addr_trans_ip_bytes: [u8; 16] = [0; 16];
        cursor.read_exact(&mut addr_trans_ip_bytes);
        let addr_trans_ip = Ipv6Addr::from(addr_trans_ip_bytes);
        let addr_trans_port = cursor.read_u16::<BigEndian>()?;

        let version = cursor.read_i32::<LittleEndian>()?;
        let version = cursor.read_i32::<LittleEndian>()?;

        let mut deserialized_version_message = VersionMessage::new();
    }
}
