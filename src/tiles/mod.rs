use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::vectors::Vector2Int;

mod renderer;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRes>()
            .add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_enter(GameState::MapInit)
                    .with_system(spawn_map)
            );
    }
}

fn spawn_map(
    mut commands: Commands,
    mut res: ResMut<TileRes>,
    assets: Res<renderer::TileAssets>
) {
    let mut tiles = HashMap::new();
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((
                    Tile::new(v),
                    renderer::get_tile_renderer(v, assets.as_ref())
                ))
                .id();
            tiles.insert(v, tile);
        }
    }
    res.tiles = tiles;
}

pub fn shift_tiles(
    dir: Vector2Int,
    query: &mut Query<&mut Tile>,
    res: &mut TileRes
) {
    let (base, offset) = match dir {
        Vector2Int::LEFT | Vector2Int::RIGHT => {
            (Vector2Int::new(MAP_SIZE/2, 0), Vector2Int::new(0, 1))
        },
        Vector2Int::UP | Vector2Int::DOWN => {
            (Vector2Int::new(0, MAP_SIZE/2), Vector2Int::new(1, 0))
        },
        _ => return
    };

    let mut new_tiles = res.tiles.clone();
    
    for i in 0..MAP_SIZE {
        for j in 0..=MAP_SIZE/2 {
            let start_v = base + i * offset + j * dir;
            let target_v = match j {
                0 => start_v + MAP_SIZE/2 * dir,
                _ => start_v - dir
            };
            let e = res.tiles[&start_v];
            new_tiles.insert(target_v, e);
            if let Ok(mut t) = query.get_mut(e) { t.v = target_v; }
        }
    }
    res.tiles = new_tiles;
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