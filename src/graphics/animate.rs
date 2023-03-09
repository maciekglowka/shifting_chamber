use bevy::prelude::*;

use crate::globals::{MAX_ANIMATION_DIST, PIECE_Z, TILE_Z};
use crate::manager::{CommandEvent, CommandType};
use crate::pieces::components::{Piece, Projectile};
use crate::states::GameState;
use crate::tiles::Tile;
use crate::vectors::Vector2Int;

use super::components::{PieceRenderer, ProjectileRenderer, TileRenderer};

const MOVEMENT_SPEED: f32 = 20.;
const PROJECTILE_SPEED: f32 = 40.;
const PROJECTILE_HEIGHT: f32 = 16.;

pub fn update_state(
    mut res: ResMut<AnimationRes>,
    mut ev_command: EventWriter<CommandEvent>
) {
    if res.is_animating {
        res.is_animating = false;
    } else {
        ev_command.send(CommandEvent(CommandType::AnimationEnd));
    }
}

pub fn update_pieces(
    mut renderer_query: Query<(&PieceRenderer, &mut Transform)>,
    piece_query: Query<&Parent, With<Piece>>,
    tile_query: Query<&Tile>,
    time: Res<Time>,
    mut res: ResMut<AnimationRes>,
) {
    let mut animating = false;
    for (renderer, mut transform) in renderer_query.iter_mut() {
        let Ok(piece_parent) = piece_query.get(renderer.target) else { continue };
        let Ok(tile) = tile_query.get(piece_parent.get()) else { continue };
        if move_towards(tile.v, PIECE_Z, &mut transform, time.as_ref()) {
            animating = true
        }
    }
    if animating {
        res.is_animating = true;
    }
}

pub fn update_tiles(
    time: Res<Time>,
    mut renderer_query: Query<(&TileRenderer, &mut Transform)>,
    tile_query: Query<&Tile>,
    mut res: ResMut<AnimationRes>,
) {
    let mut animating = false;
    for (renderer, mut transform) in renderer_query.iter_mut() {
        let Ok(tile) = tile_query.get(renderer.target) else { continue };
        if move_towards(tile.v, TILE_Z, &mut transform, time.as_ref()) {
            animating = true
        }
    }
    if animating {
        res.is_animating = true;
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut renderer_query: Query<(Entity, &mut ProjectileRenderer, &mut Transform)>,
    projectile_query: Query<&Projectile>,
    mut res: ResMut<AnimationRes>,
) {
    let mut animating = false;
    for (entity, mut renderer, mut transform) in renderer_query.iter_mut() {
        let Ok(projectile) = projectile_query.get(renderer.target) else { continue };
        let source = super::get_projectile_base_position(projectile.source);
        let target = super::get_projectile_base_position(projectile.target);
        let d = (target - renderer.linear_position).length();
        if d > MAX_ANIMATION_DIST {
            let total = (target - source).length();
            let progress = 1. - (d / total);
            renderer.linear_position = renderer.linear_position.lerp(
                target,
                progress.max(0.1) * PROJECTILE_SPEED * time.delta_seconds()
            );
            transform.translation = renderer.linear_position
                + Vec3::new(0., PROJECTILE_HEIGHT * (progress * std::f32::consts::PI).sin(), 0.);
            animating = true;
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
    if animating {
        res.is_animating = true;
    }
}

fn move_towards(
    v: Vector2Int,
    z: f32,
    transform: &mut Transform,
    time: &Time
) -> bool {
    let target = super::get_world_position(v, z);
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

#[derive(Default, Resource)]
pub struct AnimationRes {
    pub is_animating: bool
}