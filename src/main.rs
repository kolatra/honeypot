#![allow(
    dead_code, // shut up clippy
)]

use std::{net::SocketAddr, sync::Arc};
use config::Config;
use dotenvy::dotenv;
use log::debug;
use valence::{
    network::{async_trait, CleanupFn, HandshakeData, ServerListPing},
    prelude::*,
};

mod config;
mod response;
mod webhook;

mod db;
mod models;
mod schema;
mod server;

pub struct GlobalData {
    pub config: Config,
    //pub db: PgConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let global = Arc::new(GlobalData {
        config: Config::new()?,
        //db: db::connect().await?
    });

    let address = format!("0.0.0.0:{}", global.config.port).parse()?;
    debug!("Listening on {}", address);

    let settings = NetworkSettings {
        connection_mode: ConnectionMode::Offline,
        address,
        callbacks: ALittleLying.into(),
        ..Default::default()
    };

    App::new()
        .insert_resource(settings)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, server::setup)
        .add_systems(Update, server::init_clients)
        .run();

    Ok(())
}

struct ALittleLying;

#[async_trait]
impl NetworkCallbacks for ALittleLying {
    async fn server_list_ping(
        &self,
        _shared: &SharedNetworkState,
        remote_addr: SocketAddr,
        handshake_data: &HandshakeData,
    ) -> ServerListPing {
        webhook::log_mc_ping(remote_addr, &handshake_data.server_address)
            .await
            .ok();

        response::base()
    }

    async fn login(
        &self,
        _shared: &SharedNetworkState,
        info: &NewClientInfo,
    ) -> Result<CleanupFn, Text> {
        dbg!(info);

        webhook::log_join(info.ip, &info.username)
            .await
            .ok();

        Ok(Box::new(move || {}))
    }
}
