use bitcoin_lightnode::network::message::factory::MessageFactory;
use bitcoin_lightnode::network::message::types;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
#[tokio::main]
async fn main() {
    let ipv4_recv: Ipv4Addr = "144.202.14.93".parse().unwrap();
    let magic = 0xD9B4BEF9;
    let command = "version";
    let payload = types::VersionMessage::default().with_addr_recv(ipv4_recv);
    let serialized_version_message =
        MessageFactory::new_serialized_message(magic, command, &payload);
    println!(
        "serialized_version_message:{:?}",
        serialized_version_message
    );

    // 尝试连接到远程节点
    let mut stream = TcpStream::connect("144.202.14.93:8333")
        .await
        .expect("Could not connect to the node");

    // 发送消息
    stream
        .write_all(&serialized_version_message)
        .await
        .expect("Failed to send message");

    // 接收并处理对方节点返回的响应
    let mut response = [0u8; 1024]; // 用于存储响应的缓冲区
    let bytes_read = stream
        .read(&mut response)
        .await
        .expect("Failed to read response");

    // 将响应字节转换为字符串并打印
    let response_str = String::from_utf8_lossy(&response[..bytes_read]);
    println!("Received response: {}", response_str);
}
