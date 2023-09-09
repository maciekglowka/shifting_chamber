use bevy::prelude::*;
use std::collections::VecDeque;

use crate::data::DataAssets;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

pub mod components;
mod placement;
mod systems;

#[derive(Event)]
pub struct PieceEvent(pub PieceEventKind);
pub enum PieceEventKind {
    Kill(Entity, Vector2Int)
}

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceRes>()
            .add_event::<PieceEvent>()
            .add_systems(OnExit(GameState::MapInit), placement::generate_pieces)
            .add_systems(
                OnEnter(GameState::TurnStart),
                (systems::walking::plan_moves, systems::queue::plan_queue)
            )
            .add_systems(
                OnEnter(GameState::NPCAction),
                (systems::projectile::launch_projectiles, systems::walking::move_walking)
                    .chain()
            )
            .add_systems(
                OnEnter(GameState::NPCResult),
                (
                    systems::walking::walk_damage,
                    systems::walking::walk_back,
                    systems::projectile::hit_projectiles
                )
            )
            .add_systems(OnExit(GameState::NPCResult), systems::queue::update_queue)
            .add_systems(
                Update,
                (systems::health::init_health, systems::health::kill_units)
                    .chain()
            )
            .add_systems(
                PostUpdate,
                systems::projectile::explode_projectiles
            );
    }
}

#[derive(Default, Resource)]
pub struct PieceRes {
    pub action_queue: VecDeque<Entity>
}

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
