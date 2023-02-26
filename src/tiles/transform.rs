use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::MAP_SIZE;
use crate::vectors::Vector2Int;
use super::{Tile, TileRes};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileTransform {
    Shift(Vector2Int),
    Switch(Vector2Int),
    Rotate(bool)
}

pub fn can_transform(
    transform: TileTransform,
    player_v: Vector2Int,
    res: &TileRes
) -> bool {
    match transform {
        TileTransform::Shift(_) => true,
        TileTransform::Switch(dir) => res.tiles.contains_key(&(player_v + dir)),
        TileTransform::Rotate(_) => 
            res.tiles.contains_key(&(player_v + Vector2Int::new(-1, -1))) &&
            res.tiles.contains_key(&(player_v + Vector2Int::new(1, 1)))
    }
}

pub fn execute(
    transform: TileTransform,
    player_v: Vector2Int,
    tile_query: &mut Query<&mut Tile>,
    res: &mut TileRes
) {
    let vs = match transform {
        TileTransform::Shift(dir) => get_shift(player_v, dir),
        TileTransform::Switch(dir) => get_switch(player_v, dir),
        TileTransform::Rotate(clockwise) => get_rotate(player_v, clockwise)
    };
    let current_tiles = res.tiles.clone();

    for (v, new_v) in vs.iter() {
        let entity = current_tiles[&v];
        if let Ok(mut tile) = tile_query.get_mut(entity) { tile.v = *new_v; }
        res.tiles.insert(*new_v, entity);
    }
}

fn get_shift(
    origin: Vector2Int,
    dir: Vector2Int
) -> HashMap<Vector2Int, Vector2Int> {
    let base = match dir {
        Vector2Int::DOWN => Vector2Int::new(origin.x, MAP_SIZE-1),
        Vector2Int::UP => Vector2Int::new(origin.x, 0),
        Vector2Int::RIGHT => Vector2Int::new(0, origin.y),
        Vector2Int::LEFT => Vector2Int::new(MAP_SIZE-1, origin.y),
        _ => return HashMap::new()
    };
    let mut output = HashMap::new();
    for i in 0..MAP_SIZE {
        let v = base + i * dir;
        let new_v = match i {
            a if a < MAP_SIZE-1 => v + dir,
            _ => base
        };
        output.insert(v, new_v);
    }
    output
}

fn get_switch(
    origin: Vector2Int,
    dir: Vector2Int
) -> HashMap<Vector2Int, Vector2Int> {
    let (base, offset) = match dir {
        Vector2Int::LEFT | Vector2Int::RIGHT => {
            (Vector2Int::new(origin.x, 0), Vector2Int::new(0, 1))
        },
        Vector2Int::UP | Vector2Int::DOWN => {
            (Vector2Int::new(0, origin.y), Vector2Int::new(1, 0))
        },
        _ => return HashMap::new()
    };
    let mut output = HashMap::new();

    for i in 0..MAP_SIZE {
        let v0 = base + i * offset;
        let v1 = v0 + dir;
        output.insert(v0, v1);
        output.insert(v1, v0);
    }

    output
}

fn get_rotate(
    origin: Vector2Int,
    clockwise: bool
) -> HashMap<Vector2Int, Vector2Int> {
    let mut output = HashMap::new();
    for x in -1..=1 {
        for y in -1..=1 {
            let base = Vector2Int::new(x, y);
            let rotated = match clockwise {
                true => Vector2Int::new(y ,-x),
                false => Vector2Int::new(-y ,x)
            };
            output.insert(origin + base, origin + rotated);
        }
    }
    output
}