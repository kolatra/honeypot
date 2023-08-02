use std::net::SocketAddr;

use webhook::client::WebhookClient;

use crate::Result;

pub async fn log_mc_ping(address: SocketAddr, target: &str) -> Result<()> {
    let msg = format!("minecraft ping from {address} to {target}");
    send(&msg).await
}

pub async fn log_connection(address: SocketAddr) -> Result<()> {
    let msg = format!("non-minecraft connection from {address}");
    send(&msg).await
}

pub async fn log_join(address: SocketAddr) -> Result<()> {
    let msg = format!("attempted join from {address}");
    send(&msg).await
}

pub async fn send(msg: &str) -> Result<()> {
    let url = std::env::var("WEBHOOK")?;
    let client = WebhookClient::new(&url);

    match client.send(|m| m.content(msg)).await {
        Ok(_) => println!("[+] sent webhook"),
        Err(e) => println!("[!] error sending webhook: {}", e),
    };

    Ok(())
}
