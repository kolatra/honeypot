#![feature(async_closure)]
use anyhow::bail as nope;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use log::info;
use once_cell::sync::Lazy;
use valence::prelude::*;

mod config;
mod response;
mod server;
mod webhook;

mod db;
mod models;
mod schema;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use config::Config;
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Could not load config"));

fn run_migrations() -> anyhow::Result<()> {
    let mut conn = db::connect().expect("Could not connect to DB");
    if let Err(e) = conn.run_pending_migrations(MIGRATIONS) {
        nope!("Error running migrations: {}", e);
    }
    Ok(info!("Migrations run successfully"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    run_migrations()?;

    let address = format!("0.0.0.0:{}", CONFIG.port).parse()?;
    info!("Listening on {}", address);

    let settings = NetworkSettings {
        connection_mode: ConnectionMode::Offline,
        address,
        callbacks: server::ALittleLying.into(),
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
