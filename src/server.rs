use valence::prelude::*;

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