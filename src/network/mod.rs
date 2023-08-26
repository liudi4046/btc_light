use std::net::Ipv4Addr;

pub mod message;
//await返回一个result，?遇到Err直接返回Err，否则返回ok中的值
pub async fn fetch_public_ip() -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    let ip_string = reqwest::get("http://api.ipify.org").await?.text().await?;
    let public_ip_from: Ipv4Addr = ip_string.parse()?;
    Ok(public_ip_from)
}
