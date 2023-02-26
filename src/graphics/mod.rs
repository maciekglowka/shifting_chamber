use bevy::prelude::*;

use crate::states::GameState;

mod animate;
mod assets;
mod components;
mod spawn;

pub use components::PieceRenderer;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(spawn::spawn_piece_renderer)
            .add_system(spawn::spawn_tile_renderer)
            .add_system_to_stage(
                CoreStage::PostUpdate, spawn::despawn_piece_renderer
            )
            .add_system_to_stage(
                CoreStage::PostUpdate, spawn::despawn_tile_renderer
            )
            .add_system_set(
                SystemSet::on_update(GameState::TurnStart)
                    .with_system(animate::update_pieces)
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
                    .with_system(animate::update_pieces)
            );
    }
}
