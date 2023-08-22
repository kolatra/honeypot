use std::net::IpAddr;

use webhook::client::WebhookClient;

pub fn log_mc_ping(address: &str, target: &str) {
    if target.starts_with("192.168") {
        return;
    }

    let msg = format!("ping: {} with target {target}", ip_info(address));
    send(msg);
}

pub fn log_join(address: IpAddr, user: &str) {
    let addr = address.to_string();
    if addr.starts_with("192.168") {
        return;
    }

    let msg = format!(
        "({}) {user} joined the server",
        ip_info(&addr)
    );
    send(msg);
}

pub fn log_leave(address: IpAddr, user: &str) {
    let msg = format!("({}) {user} disconnected", ip_info(&address.to_string()));
    send(msg);
}

fn send(msg: String) {
    if cfg!(debug_assertions) {
        return;
    }

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
        ip = &ip[..i];
    };

    format!("[`{ip}`](<https://ipinfo.io/{ip}>)")
}

#[test]
fn test_ip_info() {
    let ip = "192.168.1.74";

    assert_eq!(
        ip_info(&format!("{ip}:25565")),
        format!("[`{ip}`](<https://ipinfo.io/{ip}>)")
    );
}
