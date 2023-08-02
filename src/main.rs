use std::{process, sync::Arc};

use config::Config;
use tokio::net::TcpListener;

pub type Result<T> = anyhow::Result<T>;

mod config;
mod handler;
mod templates;
mod webhook;

pub struct GlobalData {
    pub config: Config,
}

async fn run() -> Result<()> {
    let global = Arc::new(GlobalData {
        config: Config::read().await?,
    });

    let addr = format!("0.0.0.0:{}", global.config.port);
    let listener = TcpListener::bind(&addr).await?;
    println!("[*] listening on {}", addr);

    let mut join_handles = vec![];
    while let Ok((incoming, _)) = listener.accept().await {
        println!(
            "[+] incoming connection from {}",
            incoming.peer_addr().unwrap()
        );

        let data_ptr = Arc::clone(&global);
        let jh = tokio::spawn(async move {
            if let Err(e) = handler::scare_away(data_ptr, incoming).await {
                println!("[!] error: {}", e)
            };
        });

        join_handles.push(jh);
    }

    for jh in join_handles {
        jh.await?;
    }

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let result = run().await;

    match result {
        Err(e) => {
            println!("error: {}", e);
            process::exit(1);
        }

        Ok(()) => process::exit(0),
    }
}
