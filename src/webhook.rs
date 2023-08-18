use std::net::IpAddr;

use webhook::client::WebhookClient;

pub async fn log_mc_ping(address: &str, target: &str) -> anyhow::Result<()> {
    let msg = format!("minecraft ping from {address} to {target}");
    send(&msg).await
}

pub async fn log_connection(address: &str) -> anyhow::Result<()> {
    let msg = format!("non-minecraft connection from {address}");
    send(&msg).await
}

pub async fn log_join(address: IpAddr, user: &str) -> anyhow::Result<()> {
    let msg = format!("({address}) {user} joined the server");
    send(&msg).await
}

pub async fn log_leave(address: IpAddr, user: &str) -> anyhow::Result<()> {
    let msg = format!("({address}) {user} disconnected");
    send(&msg).await
}

pub async fn send(msg: &str) -> anyhow::Result<()> {
    let url = std::env::var("WEBHOOK")?;
    let client = WebhookClient::new(&url);

    match client.send(|m| m.content(msg)).await {
        Ok(_) => println!("[+] sent webhook"),
        Err(e) => println!("[!] error sending webhook: {}", e),
    };

    Ok(())
}
