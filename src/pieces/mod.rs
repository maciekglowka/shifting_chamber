use bevy::prelude::*;
use std::collections::VecDeque;

use crate::data::DataAssets;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

pub mod components;
mod placement;
mod systems;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceRes>()
            .add_system(placement::generate_pieces.in_schedule(OnExit(GameState::MapInit)))
            .add_systems(
                (systems::walking::plan_moves, systems::queue::plan_queue)
                .in_schedule(OnEnter(GameState::TurnStart))
            )
            .add_systems(
                (systems::walking::move_walking, systems::projectile::launch_projectiles)
                .in_schedule(OnEnter(GameState::NPCAction)))
            .add_systems(
                (
                    systems::walking::walk_damage,
                    systems::walking::walk_back,
                    systems::projectile::hit_projectiles
                )
                .in_schedule(OnEnter(GameState::NPCResult))
            )
            .add_system(systems::queue::update_queue.in_schedule(OnExit(GameState::NPCResult)))
            .add_system(systems::health::kill_units);
    }
}

#[derive(Default, Resource)]
pub struct PieceRes {
    pub action_queue: VecDeque<Entity>
}

// pub fn spawn_piece_at_entity(
//     commands: &mut Commands,
//     name: String,
//     parent_entity: Entity,
//     data_assets: &DataAssets
// ) {
//     let entity = get_new_piece(commands, name, data_assets);
//     commands.entity(parent_entity)
//         .push_children(&[entity]);
// }

pub fn spawn_piece_at_v(
    commands: &mut Commands,
    name: String,
    v: Vector2Int,
    tile_res: &TileRes,
    data_assets: &DataAssets
) -> Entity {
    let entity = get_new_piece(commands, name, data_assets);
    commands.entity(tile_res.tiles[&v])
        .push_children(&[entity]);
    entity
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
