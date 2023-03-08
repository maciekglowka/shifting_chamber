use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub mod transform;
pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRes>()
            .add_system(spawn_map.in_schedule(OnEnter(GameState::MapInit)));
    }
}

fn spawn_map(
    mut commands: Commands,
    query: Query<Entity, With<Tile>>,
    mut res: ResMut<TileRes>
) {
    clear_map(&mut commands, &query);

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
    query: &Query<Entity, With<Tile>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
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