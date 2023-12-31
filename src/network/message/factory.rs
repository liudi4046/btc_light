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
        serialized_message.extend(&self.length.to_le_bytes());
        serialized_message.extend(&self.checksum);
        serialized_message
    }
}
pub struct MessageFactory;

impl MessageFactory {
    //从收到的原始消息创建消息结构体
    pub fn from_raw(data: &[u8], msg_type: MessageType) -> Result<Box<dyn Message>, &'static str> {
        let mut cursor = Cursor::new(data);

        match msg_type {
            MessageType::Version => {
                let version_msg = VersionMessage::deserialize(&mut cursor)?;
                Ok(Box::new(version_msg))
            }
            MessageType::Verack => {
                let verack_msg = VerackMessage::deserialize(&mut cursor)?;
                Ok(Box::new(verack_msg))
            }
            // ... 其他消息类型的处理
            _ => Err("Unsupported message type"),
        }
    }

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

        let serialized_message_header = message_header.serialize();

        println!(
            "serialized_message_header: {}",
            serialized_message_header
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<String>>()
                .join(" ")
        );

        let serialized_payload = payload.serialize();
        println!(
            "serialized_payload: {}",
            serialized_payload
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<String>>()
                .join(" ")
        );

        serialized_message_header
            .into_iter()
            .chain(serialized_payload.into_iter())
            .collect()
    }
}

pub trait Message {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

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
