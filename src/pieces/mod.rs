use bevy::prelude::*;
use rand::prelude::*;

use crate::actions::ActionKind;
use crate::data::DataAssets;
use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

pub mod components;
mod placement;
pub mod renderer;
mod systems;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_exit(GameState::MapInit)
                    .with_system(placement::generate_pieces)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PlayerInput)
                    .with_system(systems::items::examine_pickable_items)
                    .label("action")
            )
            .add_system_set(
                SystemSet::on_enter(GameState::ShiftResult)
                    .with_system(systems::fights::check_fights)
                    .with_system(systems::interactions::check_instant)
                    .with_system(systems::interactions::check_interactions)
                    .with_system(systems::interactions::check_damage)
                )
                .add_system_set(
                    SystemSet::on_exit(GameState::ShiftResult)
                    .with_system(systems::fights::kill_units)
                    .with_system(systems::items::update_temporary)
                    .with_system(systems::items::remove_disposable_items)
            );
    }
}

pub fn spawn_piece_at_entity(
    commands: &mut Commands,
    name: String,
    parent_entity: Entity,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) {
    let entity = get_new_piece(commands, name, assets, data_assets);
    commands.entity(parent_entity)
        .push_children(&[entity]);
}

fn spawn_piece_at_v(
    commands: &mut Commands,
    name: String,
    v: Vector2Int,
    tile_res: &TileRes,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) {
    let entity = get_new_piece(commands, name, assets, data_assets);
    commands.entity(tile_res.tiles[&v])
        .push_children(&[entity]);
}

fn get_new_piece(
    commands: &mut Commands,
    name: String,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) -> Entity {
    let (data, component_list) = components::get_piece_data(&name, data_assets);
    
    let mut piece = commands.spawn((
        components::Piece,
        renderer::get_piece_renderer(&data["sprite"], &assets)
    ));

    if component_list.contains_key("Effect") {
        // when spawning an effect, wrap it inside an instant item
        piece.insert((
            components::Item,
            components::Instant {
                kind: ActionKind::ApplyEffect(name)
            }
        ));
    } else {
        // otherwise just build component list normally
        components::insert_from_list(&mut piece, component_list);
    }   
    piece.id()
}
