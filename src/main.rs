use bitcoin_lightnode::network::message::factory::MessageFactory;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

fn main() {
    // let version_message = MessageFactory::new_version();
    let ipv4_recv: Ipv4Addr = "142.113.247.76".parse().unwrap();
    let version_message = MessageFactory::new_version(Some(ipv4_recv));
    println!("version_message:{:?}", version_message);
}
