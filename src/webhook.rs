use std::net::IpAddr;

use webhook::client::WebhookClient;

pub fn log_mc_ping(address: &str, target: &str) {
    let msg = format!("minecraft ping from {address} to {target}");
    send(msg)
}

pub fn log_connection(address: &str) {
    let msg = format!("non-minecraft connection from {address}");
    send(msg)
}

pub fn log_join(address: IpAddr, user: &str) {
    let msg = format!("({address}) {user} joined the server");
    send(msg)
}

pub fn log_leave(address: IpAddr, user: &str) {
    let msg = format!("({address}) {user} disconnected");
    send(msg)
}

fn send(msg: String) {
    tokio::task::spawn(async move {
        let url = std::env::var("WEBHOOK")?;
        let client = WebhookClient::new(&url);

        match client.send(|m| m.content(&msg)).await {
            Ok(_) => println!("[+] sent webhook"),
            Err(e) => println!("[!] error sending webhook: {}", e),
        };

        Ok::<(), anyhow::Error>(())
    });
}
