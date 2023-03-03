use bevy::prelude::*;

use crate::states::GameState;
use crate::globals::{MAP_SIZE, Y_PERSPECTIVE, TILE_SIZE, TILE_Z};
use crate::vectors::Vector2Int;

mod animate;
mod assets;
mod components;
mod spawn;

pub use components::PieceRenderer;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .init_resource::<animate::AnimationRes>()
            .add_system(spawn::spawn_piece_renderer)
            .add_system(spawn::spawn_tile_renderer)
            .add_system(spawn::spawn_projectile_renderer)
            .add_system_to_stage(
                CoreStage::PostUpdate, animate::update_state
            )
            .add_system_to_stage(
                CoreStage::PostUpdate, spawn::despawn_piece_renderer
            )
            .add_system_to_stage(
                CoreStage::PostUpdate, spawn::despawn_tile_renderer
            )
            .add_system_set(
                SystemSet::on_update(GameState::TileShift)
                    .with_system(animate::update_tiles)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::NPCMove)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::MoveResult)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::TurnEnd)
                    .with_system(animate::update_projectiles)
            );
    }
}

const TILE_VARIANTS: usize = 4;
const SPRITE_SIZE: f32 = 32.;
const PIECE_FRAMES: usize = 4;

pub fn get_world_position(v: Vector2Int, z: f32) -> Vec3 {
    let offset = if z == TILE_Z { 0. } else {TILE_SIZE * 0.25};
    Vec3::new(
        v.x as f32 * TILE_SIZE,
        v.y as f32 * TILE_SIZE * Y_PERSPECTIVE + offset,
        z + (MAP_SIZE - v.y) as f32)
}