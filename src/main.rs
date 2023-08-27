use bitcoin_lightnode::network::fetch_public_ip;
use bitcoin_lightnode::network::message::factory::MessageFactory;
use bitcoin_lightnode::network::message::types;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
#[tokio::main]
async fn main() {
    let ipv4_recv: Ipv4Addr = "43.159.49.47".parse().unwrap();

    let ipv4_from = fetch_public_ip().await.expect("fetch public ip error");

    let magic = 0xD9B4BEF9;
    let command = "version";
    let payload = types::VersionMessage::default()
        .with_addr_recv(ipv4_recv)
        .with_addr_from(ipv4_from);
    let serialized_version_message =
        MessageFactory::new_serialized_message(magic, command, &payload);
    println!(
        "serialized_version_message: {}",
        serialized_version_message
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    );
    let full_sample_data = [
        0xd9, 0xb4, 0xbe, 0xf9, // Magic value
        0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00,
        0x00, // Command name: "version"
        0x5d, 0x00, 0x00, 0x00, // Payload length: 93
        0x1, 0x2, 0x3, 0x4, // Checksum (你需要自己计算)
        0x72, 0x11, 0x01, 0x00, // Protocol version
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Services: NODE_NETWORK
        0xbc, 0x8f, 0x5e, 0x54, 0x00, 0x00, 0x00, 0x00, // [Epoch time][unix epoch time]
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Receiving node's services
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xc6, 0x1b, 0x64,
        0x09, // Receiving node's IPv6 address
        0x20, 0x8d, // Receiving node's port number
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Transmitting node's services
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xcb, 0x00, 0x71,
        0xc0, // Transmitting node's IPv6 address
        0x20, 0x8d, // Transmitting node's port number
        0x12, 0x80, 0x35, 0xcb, 0xc9, 0x79, 0x53, 0xf8, // Nonce
        0x0f, // Bytes in user agent string
        0x2f, 0x53, 0x61, 0x74, 0x6f, 0x73, 0x68, 0x69, 0x3a, 0x30, 0x2e, 0x39, 0x2e, 0x33,
        0x2f, // User agent
        0xcf, 0x05, 0x05, 0x00, // Start height
        0x01, // Relay flag
    ];
    // 尝试连接到远程节点
    let mut stream = TcpStream::connect("43.159.49.47:8333")
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
    println!(
        "original response: {}",
        response
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    );

    let response_str = String::from_utf8_lossy(&response[..bytes_read]);
    println!("Received response: {}", response_str);
}
