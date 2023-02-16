use bevy::prelude::*;
use std::collections::VecDeque;

use crate::data::DataAssets;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

pub mod components;
mod placement;
// pub mod renderer;
mod systems;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceRes>()
            .add_system_set(
                SystemSet::on_exit(GameState::MapInit)
                    .with_system(placement::generate_pieces)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PlayerInput)
                    .with_system(systems::units::plan_moves)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::NPCMove)
                    .with_system(systems::units::move_walking)
                )
            .add_system_set(
                SystemSet::on_enter(GameState::TurnEnd)
                    .with_system(systems::units::kill_units)
                );
    }
}

#[derive(Default, Resource)]
pub struct PieceRes {
    pub walking_queue: VecDeque<Entity>
}

// pub fn spawn_piece_at_entity(
//     commands: &mut Commands,
//     name: String,
//     parent_entity: Entity,
//     // assets: &renderer::PieceAssets,
//     data_assets: &DataAssets
// ) {
//     let entity = get_new_piece(commands, name, data_assets);
//     commands.entity(parent_entity)
//         .push_children(&[entity]);
// }

fn spawn_piece_at_v(
    commands: &mut Commands,
    name: String,
    v: Vector2Int,
    tile_res: &TileRes,
    data_assets: &DataAssets
) {
    let entity = get_new_piece(commands, name, data_assets);
    commands.entity(tile_res.tiles[&v])
        .push_children(&[entity]);
}

fn get_new_piece(
    commands: &mut Commands,
    name: String,
    data_assets: &DataAssets
) -> Entity {
    let data_item = &data_assets.pieces[&name];
    
    let mut piece = commands.spawn((
            Name::new(name.to_string()),
            components::Piece { name },
        ));

    components::insert_from_list(&mut piece, &data_item.components);
    piece.id()
}
