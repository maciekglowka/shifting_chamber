use bevy::prelude::*;

use crate::globals::{MAX_ANIMATION_DIST, PIECE_Z, TILE_SIZE, TILE_Z};
use crate::manager::{CommandEvent, CommandType};
use crate::pieces::components::Piece;
use crate::tiles::Tile;
use crate::vectors::Vector2Int;

use super::components::{PieceRenderer, TileRenderer};

const MOVEMENT_SPEED: f32 = 20.;

pub fn update_pieces(
    mut renderer_query: Query<(&PieceRenderer, &mut Transform)>,
    piece_query: Query<&Parent, With<Piece>>,
    tile_query: Query<&Tile>,
    time: Res<Time>,
    mut ev_command: EventWriter<CommandEvent>
) {
    let mut animating = false;
    for (renderer, mut transform) in renderer_query.iter_mut() {
        let Ok(piece_parent) = piece_query.get(renderer.target) else { continue };
        let Ok(tile) = tile_query.get(piece_parent.get()) else { continue };
        if move_towards(tile.v, PIECE_Z, &mut transform, time.as_ref()) {
            animating = true
        }
    }
    if !animating {
        ev_command.send(CommandEvent(CommandType::AnimationEnd));
    }
}

pub fn update_tiles(
    time: Res<Time>,
    mut renderer_query: Query<(&TileRenderer, &mut Transform)>,
    tile_query: Query<&Tile>,
) {
    for (renderer, mut transform) in renderer_query.iter_mut() {
        let Ok(tile) = tile_query.get(renderer.target) else { continue };
        move_towards(tile.v, TILE_Z, &mut transform, time.as_ref());
    }
}

fn move_towards(
    v: Vector2Int,
    z: f32,
    transform: &mut Transform,
    time: &Time
) -> bool {
    let target = Vec3::new(v.x as f32 * TILE_SIZE, v.y as f32 * TILE_SIZE, z);
    let d = (target - transform.translation).length();
    if d > MAX_ANIMATION_DIST {
        transform.translation = transform.translation.lerp(
            target,
            MOVEMENT_SPEED * time.delta_seconds()
        );
        return true;
    }
    transform.translation = target;
    false
}