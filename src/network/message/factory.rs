use super::types::{NetAddr, VersionMessage};
use sha2::{Digest, Sha256};
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
    pub fn new<T: Message>(magic: u32, cmd: &str, payload: &T) -> Self {
        let mut command = [0u8; 12];
        command[..cmd.len()].copy_from_slice(cmd.as_bytes());

        let serialized_payload = payload.serialize();
        let length = serialized_payload.len() as u32;
        let checksum = checksum(&serialized_payload);

        MessageHeader {
            magic,
            command,
            length,
            checksum,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized_message: Vec<u8> = Vec::new();
        serialized_message.extend(&self.magic.to_le_bytes());
        serialized_message.extend(&self.command);
        serialized_message.extend(&self.length.to_be_bytes());
        serialized_message.extend(&self.checksum);
        serialized_message
    }
}
pub struct MessageFactory;

impl MessageFactory {
    // 根据给定参数创建新消息
    pub fn new_version_payload(ipv4_recv: Option<Ipv4Addr>) -> VersionMessage {
        match ipv4_recv {
            Some(ip) => VersionMessage::default().with_addr_recv(ip),
            None => VersionMessage::default(),
        }
    }

    pub fn new_serialized_message<T: Message + std::fmt::Debug>(
        magic: u32,
        command: &str,
        payload: &T,
    ) -> Vec<u8> {
        let message_header = MessageHeader::new(magic, command, payload);
        println!("message_header:{:?}", message_header);
        println!("payload:{:?}", payload);
        let serialized_message_header = message_header.serialize();
        let serialized_payload = payload.serialize();
        serialized_message_header
            .into_iter()
            .chain(serialized_payload.into_iter())
            .collect()
    }
}

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
}

// fn create_header<T>(payload: &T) -> MessageHeader {
//     let length = payload.len() as u32;

//     if payload.command
//     let checksum = calculate_checksum(&payload); // 假设这个函数计算校验和

//     // 你可以从 message 对象中获取命令名称，或者使用 Rust 的类型系统来确定它
//     let command = get_command_name(&message); // 假设这个函数返回命令名称

//     MessageHeader::new(0xD9B4BEF9, &command, length, checksum)
// }

// fn serialize<T>(payload: &T) -> Vec<u8>
// where
//     T: Message,
// {
//     let message_header = create_header(payload);
//     let serialized_message_header = message_header.serialize();
//     let serialized_payload = payload.serialize();
//     let mut res = Vec::new();
//     res.extend(serialized_message_header);
//     res.extend(serialized_payload);
//     res

//     //serialize payload
// }

fn checksum(payload: &[u8]) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let first_hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&first_hash);
    let second_hash = hasher.finalize();

    let mut result = [0u8; 4];
    result.copy_from_slice(&second_hash[0..4]);
    result
}
