use bevy::prelude::*;

use crate::actions::ActionKind;
use crate::data::DataAssets;
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
                // these systems would fire before eg. damage is applied
                // so it is possible to grab healing potion and not die the same turn :)
                SystemSet::on_enter(GameState::TileShift)
                    .with_system(systems::interactions::check_instant)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::ShiftResult)
                    .with_system(systems::units::check_unit_damage)
                    .with_system(systems::units::check_poisoning)
                    .with_system(systems::units::apply_poison)
                    .with_system(systems::interactions::check_interactions)
                    .with_system(systems::interactions::check_damage)
                )
            .add_system_set(
                SystemSet::on_exit(GameState::ShiftResult)
                    .with_system(systems::units::kill_units)
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
    let data_item = &data_assets.pieces[&name];
    
    let mut piece = commands.spawn((
        components::Piece,
        renderer::get_piece_renderer(&data_item.sprite, &assets)
    ));

    // if data_item.components.contains_key("Effect") {
    //     // when spawning an effect, wrap it inside an instant item
    //     piece.insert((
    //         components::Item,
    //         components::Instant {
    //             kind: ActionKind::ApplyEffect(name)
    //         }
    //     ));
    // } else {
    //     // otherwise just build component list normally
    //     components::insert_from_list(&mut piece, &data_item.components);
    // }   

    components::insert_from_list(&mut piece, &data_item.components);
    piece.id()
}
