#![allow(dead_code)]
#![feature(async_closure)]
use config::Config;
use diesel::PgConnection;
use dotenvy::dotenv;
use log::{debug, error, info};
use std::{net::SocketAddr, sync::Arc};
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

pub struct GlobalData {
    pub config: Config,
    pub db: PgConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let global = Arc::new(GlobalData {
        config: Config::new()?,
        db: db::connect().await?,
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
        .add_systems(Startup, setup)
        .add_systems(Update, init_clients)
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

        let mut conn = db::connect().await.expect("Could not connect to DB");
        let addr_port = remote_addr.to_string();
        let addr = &addr_port[..addr_port.len() - 6];

        match db::add_or_update(&mut conn, addr, db::Update::Ping).await {
            Ok(a) => info!("{:?}", a),
            Err(e) => error!("db error: {e}"),
        }

        webhook::log_mc_ping(&addr_port, &handshake_data.server_address);

        response::base()
    }

    async fn login(
        &self,
        _shared: &SharedNetworkState,
        info: &NewClientInfo,
    ) -> Result<CleanupFn, Text> {

        let mut conn = db::connect().await.expect("Could not connect to DB");
        let addr = info.ip;

        match db::add_or_update(&mut conn, &addr.to_string(), db::Update::Join).await {
            Ok(a) => info!("{:?}", a),
            Err(e) => error!("{e}"),
        }

        webhook::log_join(info.ip, &info.username);

        let user = info.username.clone();
        Ok(Box::new(move || {
            webhook::log_leave(addr, &user);
        }))
    }
}

const SPAWN_Y: i32 = 0;

pub fn setup(
    mut commands: Commands,
    dimensions: Res<DimensionTypeRegistry>,
    mut biomes: ResMut<BiomeRegistry>,
    server: Res<Server>,
) {
    let size = 5;

    biomes.insert(ident!("plains"), Biome::default());

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -size..size {
        for x in -size..size {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for x in -size * 16..size * 16 {
        for z in -size * 16..size * 16 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
        }
    }

    commands.spawn(layer);
}

pub fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, SPAWN_Y as f64 + 1.0, 0.0]);
        *game_mode = GameMode::Adventure;
    }
}
