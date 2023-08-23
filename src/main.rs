use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到比特币节点的地址和端口
    let node_address: SocketAddr = "86.13.240.14:8333".parse()?;

    // 建立异步TCP连接
    let mut stream = TcpStream::connect(node_address).await?;

    println!("Connected to Bitcoin node!");

    // 发送一些数据到节点
    let message = b"Hello, Bitcoin Node!";
    stream.write_all(message).await?;

    // 接收来自节点的响应
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("n{}", n);
    println!("buf:{:?}", buffer);
    println!("Received from node: {}", response);

    Ok(())
}
