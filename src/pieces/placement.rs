use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use crate::data::{DataAssets, PieceData};
use crate::globals::{MAP_SIZE, MAP_POINTS_MUL};
use crate::manager::GameRes;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

// use super::renderer::PieceAssets;

pub fn generate_pieces(
    mut commands: Commands,
    tile_res: Res<TileRes>,
    // assets: Res<PieceAssets>,
    data_assets: Res<DataAssets>,
    mut game_res: ResMut<GameRes>
) {
    let target_points = (game_res.level * MAP_POINTS_MUL) as i32;
    let level_type = get_level_type(&mut game_res, &data_assets, target_points);
    let player_v = Vector2Int::new(MAP_SIZE / 2, MAP_SIZE / 2);

    let item_pool = get_name_pool(
        &data_assets.pieces,
        &data_assets.item_names,
        game_res.level,
        false
    );
    let fixture_pool = get_name_pool(
        &data_assets.pieces,
        &data_assets.fixture_names,
        game_res.level,
        false
    );
    let unit_pool = get_name_pool(
        &data_assets.pieces,
        &data_assets.unit_names,
        game_res.level,
        true
    );

    let level_data = &data_assets.levels[&level_type];

    let mut points = level_data.initial_points;
    let mut pieces = level_data.required_pieces.clone();
    // pieces.push("Stair".to_string());
    let mut rng = thread_rng();

    // for _ in 0..rng.gen_range(level_data.extra_items.0..=level_data.extra_items.1) {
    //     let name = &item_pool.choose_weighted(&mut rng, |v| v.1).unwrap().0;
    //     points += data_assets.pieces[name].points.unwrap_or(0);
    //     pieces.push(name.to_string());
    // }

    for _ in 0..rng.gen_range(level_data.extra_features.0..=level_data.extra_features.1) {
        let name = &fixture_pool.choose_weighted(&mut rng, |v| v.1).unwrap().0;
        points += data_assets.pieces[name].points.unwrap_or(0);
        pieces.push(name.to_string());
    }

    while points < target_points {
        let name = &unit_pool.choose_weighted(&mut rng, |v| v.1).unwrap().0;
        points += data_assets.pieces[name].points.unwrap_or(0);
        pieces.push(name.to_string());
    }

    spawn_level_pieces(
        &mut commands,
        pieces,
        data_assets.as_ref(),
        player_v,
        tile_res.as_ref(),
        // assets.as_ref()
    )
}

fn spawn_level_pieces(
    commands: &mut Commands,
    pieces: Vec<String>,
    data_assets: &DataAssets,
    player_v: Vector2Int,
    tile_res: &TileRes,
    // assets: &PieceAssets
) {
    let mut tile_pool: Vec<_> = tile_res.tiles.keys()
    .filter(|a| a.manhattan(player_v) > 1)
    .map(|a| *a)
    .collect();

    for name in pieces {
        let v = match data_assets.pieces[&name].points {
            Some(a) if a > 0 => get_near_tile(&mut tile_pool, player_v),
            _ => get_far_tile(&mut tile_pool, player_v)
        };
        if let Some(v) = v {
            super::spawn_piece_at_v(
                commands,
                name.into(),
                v,
                &tile_res,
                &data_assets
            );
        }
    }
}

fn get_level_type(
    game_res: &mut GameRes,
    data_assets: &DataAssets,
    target_points: i32
) -> String {
    let possible: Vec<_> = data_assets.levels.iter()
        .filter(|(_, v)| v.initial_points <= target_points)
        .map(|(k, _)| k)
        .collect();

    let pool: Vec<_> = possible.iter()
        .map(|n| {
            let last_idx = game_res.level_history.iter()
                .rposition(|l| l == *n)
                .unwrap_or(0);
            (*n, game_res.level_history.len() - last_idx + 1)
        })
        .collect();
    
    let mut rng = thread_rng();
    let name = pool.choose_weighted(&mut rng, |n| n.1).unwrap().0;
    game_res.level_history.push(name.clone());
    name.to_owned()
}

fn get_name_pool(data: &HashMap<String, PieceData>, names: &Vec<String>, level: u32, weighted: bool) -> Vec<(String, i32)> {
    names.iter()
        .filter(|n| data[*n].points.is_some() && data[*n].min_level.unwrap_or(0) <= level)
        .map(|n| {
            let w = match weighted {
                false => 1,
                true => data[n].points.unwrap_or(1).abs()
            };
            (n.clone(), w)
        })
        .collect()
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