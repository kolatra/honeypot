use std::{process, sync::Arc};

use config::Config;
use tokio::net::TcpListener;

pub type Result<T> = anyhow::Result<T>;

mod config;

struct GlobalData {
    config: Config,
}

async fn run() -> Result<bool> {
    let global = Arc::new(GlobalData {
        config: Config::read().await?,
    });

    let addr = format!("0.0.0.0:{}", global.config.port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);

    let mut join_handles = vec![];
    while let Ok((incoming, _)) = listener.accept().await {
        let jh = tokio::spawn(
            async move { println!("Accepted connection from {}", incoming.peer_addr().unwrap()); }
        );
        join_handles.push(jh);
    }

    for jh in join_handles {
        jh.await.unwrap();
    }

    Ok(true)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let result = run().await;

    match result {
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }

        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
    }
}
