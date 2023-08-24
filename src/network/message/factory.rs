use super::types::{NetAddr, VersionMessage};
use std::{io::Cursor, net::Ipv4Addr};

pub enum MessageType {
    Version,
    Verack,
    // ... 其他消息类型
}

#[derive(Debug)]
pub struct MessageHeader {
    magic: u32,
    command: [u8; 12],
    length: u32,
    checksum: [u8; 4],
}

impl MessageHeader {
    pub fn new(magic: u32, command: &str, length: u32, checksum: [u8; 4]) -> Self {
        let mut cmd = [0u8; 12];
        cmd[..command.len()].copy_from_slice(command.as_bytes());
        MessageHeader {
            magic,
            command: cmd,
            length,
            checksum,
        }
    }
}
pub struct MessageFactory;

impl MessageFactory {
    // 从原始数据创建消息
    // pub fn from_raw(data: &[u8], msg_type: MessageType) -> Result<Box<dyn Message>, &'static str> {
    //     let mut cursor = Cursor::new(data);

    //     match msg_type {
    //         MessageType::Version => {
    //             let version_msg = VersionMessage::deserialize(&mut cursor)?;
    //             Ok(Box::new(version_msg))
    //         }
    //         MessageType::Verack => {
    //             let verack_msg = VerackMessage::deserialize(&mut cursor)?;
    //             Ok(Box::new(verack_msg))
    //         }
    //         // ... 其他消息类型的处理
    //         _ => Err("Unsupported message type"),
    //     }
    // }

    // 根据给定参数创建新消息
    pub fn new_version(ipv4_recv: Option<Ipv4Addr>) -> VersionMessage {
        match ipv4_recv {
            Some(ip) => VersionMessage::default().with_addr_recv(ip),
            None => VersionMessage::default(),
        }
    }

    // pub fn new_verack() -> VerackMessage {
    //     // ... 创建并返回一个Verack消息
    // }

    // ... 其他消息类型的工厂方法
}

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, &'static str>
    where
        Self: Sized;
}

fn create_header<M: Message>(message: &M) -> MessageHeader {
    let payload = message.serialize(); // 假设这个方法返回消息体的序列化形式
    let length = payload.len() as u32;
    let checksum = calculate_checksum(&payload); // 假设这个函数计算校验和

    // 你可以从 message 对象中获取命令名称，或者使用 Rust 的类型系统来确定它
    let command = get_command_name(&message); // 假设这个函数返回命令名称

    MessageHeader::new(0xD9B4BEF9, &command, length, checksum)
}
