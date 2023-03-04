use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::vectors::Vector2Int;

// mod renderer;
pub mod transform;
pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRes>()
            .add_system_set(
                SystemSet::on_enter(GameState::MapInit)
                    .with_system(spawn_map)
                    .after("player")
            );
    }
}

fn spawn_map(
    mut commands: Commands,
    mut res: ResMut<TileRes>
    // assets: Res<renderer::TileAssets>
) {
    clear_map(&mut commands, res.as_ref());

    let mut tiles = HashMap::new();
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn(Tile::new(v)).id();
            tiles.insert(v, tile);
        }
    }
    res.tiles = tiles;
}

fn clear_map(
    commands: &mut Commands,
    res: &TileRes
) {
    for entity in res.tiles.values() {
        commands.entity(*entity).despawn_recursive();
    }
}

#[derive(Default, Resource)]
pub struct TileRes {
    pub tiles: HashMap<Vector2Int, Entity>
}

#[derive(Component)]
pub struct Tile {
    pub v: Vector2Int
}

impl Tile {
    fn new(v: Vector2Int) -> Tile {
        Tile { v }
    }
}