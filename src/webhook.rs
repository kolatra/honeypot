use std::net::IpAddr;

use webhook::client::WebhookClient;

pub fn log_mc_ping(address: &str, target: &str) {
    let msg = format!("minecraft ping from {} to {target}", ip_info(address));
    send(msg)
}

pub fn log_connection(address: &str) {
    let msg = format!("non-minecraft connection from {}", ip_info(address));
    send(msg)
}

pub fn log_join(address: IpAddr, user: &str) {
    let msg = format!("({}) {user} joined the server", ip_info(&address.to_string()));
    send(msg)
}

pub fn log_leave(address: IpAddr, user: &str) {
    let msg = format!("({}) {user} disconnected", ip_info(&address.to_string()));
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

fn ip_info(mut ip: &str) -> String {
    if let Some(i) = ip.find(':') {
        ip = &ip[..i]
    };

    format!("[`{}`](<https://ipinfo.io/{}>)", ip, ip)
}

#[test]
fn test_ip_info() {
    let ip = "192.168.1.74";

    assert_eq!(
        ip_info(&format!("{}:25565", ip)),
        format!("[`{}`](<https://ipinfo.io/{}>)", ip, ip)
    );
}
