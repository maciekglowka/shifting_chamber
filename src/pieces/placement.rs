use bevy::prelude::*;
use rand::prelude::*;

use crate::data::DataAssets;
use crate::globals::MAP_SIZE;
use crate::manager::GameRes;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

use super::renderer::PieceAssets;

pub fn generate_pieces(
    mut commands: Commands,
    tile_res: Res<TileRes>,
    assets: Res<PieceAssets>,
    data_assets: Res<DataAssets>,
    game_res: Res<GameRes>
) {
    let target_points = (game_res.level * 2) as i32;
    let player_v = Vector2Int::new(MAP_SIZE / 2, MAP_SIZE / 2);
    let mut tile_pool: Vec<_> = tile_res.tiles.keys()
        .filter(|a| a.manhattan(player_v) > 1)
        .map(|a| *a)
        .collect();

    // let pieces = ["Stair", "Shield", "Heal", "Fire"];
    // let mut pieces = vec!("Stair");
    let piece_pool: Vec<_> = data_assets.entities.iter()
        .filter(|(_, v)| v.points.is_some() && v.min_level.unwrap_or(0) <= game_res.level)
        .map(|(k, v)| (k, v.points.unwrap().abs()))
        .collect();

    let mut points = 0;
    let mut pieces = vec!("Stair");
    let mut rng = thread_rng();

    while points < target_points {
        let v = piece_pool.choose_weighted(&mut rng, |v| v.1).unwrap().0;
        pieces.push(v);
        points += data_assets.entities[v].points.unwrap_or(0);
    }
    
    for name in pieces {
        let v = match data_assets.entities[name].points {
            Some(a) if a > 0 => get_near_tile(&mut tile_pool, player_v),
            _ => get_far_tile(&mut tile_pool, player_v)
        };
        if let Some(v) = v {
            super::spawn_piece_at_v(
                &mut commands,
                name.into(),
                v,
                &tile_res,
                &assets,
                &data_assets
            );
        }
    }

    // for _ in 0..4 {
    //     if let Some(v) = get_near_tile(&mut tile_pool, player_v) {
    //         super::spawn_piece_at_v(
    //             &mut commands, 
    //             "Mask".into(),
    //             v,
    //             &tile_res,
    //             &assets,
    //             &data_assets
    //         );
    //     }
    // }
}

fn get_far_tile(pool: &mut Vec<Vector2Int>, player_v: Vector2Int) -> Option<Vector2Int> {
    get_random_tile(pool, |v| v.manhattan(player_v))
}

fn get_near_tile(pool: &mut Vec<Vector2Int>, player_v: Vector2Int) -> Option<Vector2Int> {
    get_random_tile(pool, |v| MAP_SIZE - v.manhattan(player_v))
}

fn get_random_tile<F: Fn(Vector2Int) -> i32>(pool: &mut Vec<Vector2Int>, f: F) -> Option<Vector2Int> {
    if pool.len() == 0 { return None; }
    let choices: Vec<_> = pool.iter()
        .map(|v| (v, f(*v)))
        .collect();
    let mut rng = thread_rng();
    let v = *choices.choose_weighted(&mut rng, |v| v.1).unwrap().0;
    pool.retain(|a| *a != v);
    Some(v)
}