use bevy::prelude::*;
use bevy::hierarchy::despawn_with_children_recursive;
use std::collections::HashMap;

use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub mod transform;
pub struct TilePlugin;

#[derive(Event)]
pub struct MapSpawnedEvent;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRes>()
            .add_event::<MapSpawnedEvent>();
    }
}

// fn spawn_map(
//     mut commands: Commands,
//     query: Query<Entity, With<Tile>>,
//     mut res: ResMut<TileRes>
// ) {
//     clear_map(&mut commands, &query);

//     let mut tiles = HashMap::new();
//     for x in 0..MAP_SIZE {
//         for y in 0..MAP_SIZE {
//             let v = Vector2Int::new(x, y);
//             let tile = commands.spawn(Tile::new(v)).id();
//             tiles.insert(v, tile);
//         }
//     }
//     res.tiles = tiles;
// }

pub fn spawn_map(
    world: &mut World
) {
    clear_map(world);

    let mut tiles = HashMap::new();
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            // let tile = commands.spawn(Tile::new(v)).id();
            let tile = world.spawn(Tile::new(v)).id();
            tiles.insert(v, tile);
        }
    }
    if let Some(mut res) = world.get_resource_mut::<TileRes>() {
        res.tiles = tiles;
    }
    world.send_event(MapSpawnedEvent);
}

fn clear_map(
    world: &mut World
) {
    let entities = world.query_filtered::<Entity, With<Tile>>()
        .iter(world)
        .collect::<Vec<_>>();
    for entity in entities {
        despawn_with_children_recursive(world, entity);
    }
}


// fn clear_map(
//     commands: &mut Commands,
//     query: &Query<Entity, With<Tile>>
// ) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn_recursive();
//     }
// }

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