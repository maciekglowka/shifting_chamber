use bevy::prelude::*;
use std::cmp;

use crate::actions::ActionEvent;
use crate::pieces::components;
use crate::player::Player;
use crate::states::GameState;
use crate::tiles;
use crate::ui;

use super::{CommandEvent, CommandType, GameRes};

// all those systems are invoked by a direct player command
// they should end player_input state on success


pub fn shift_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Player>,
    unit_query: Query<&Parent, With<components::Unit>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>,
    mut ev_tile: EventWriter<tiles::TileSwapEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::MapShift(v0, v1) = ev.0 {
            if v0.manhattan(v1) != 1 { continue; }

            let player_v = player_query.get_single().unwrap().v;

            if tiles::can_shift(v0, v1-v0, player_v, &unit_query, &tile_res) {
                tiles::shift_tiles(v0, v1-v0, &mut tile_query, tile_res.as_mut(), &mut ev_tile);
                game_state.set(GameState::TileShift).expect("Switching states failed");
            }
        }
    }
}

pub fn next_level(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::NextLevel = ev.0 {
            game_state.set(GameState::MapInit).expect("Switching states failed");
        }
    }
}

pub fn upgrade(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>,
    mut ev_action: EventWriter<ActionEvent>,
    mut res: ResMut<GameRes>
) {
    for ev in ev_command.iter() {
        if let CommandType::Upgrade(kind) = &ev.0 {
            res.next_upgrade += cmp::max(2, res.next_upgrade / 2);
            ev_action.send(ActionEvent(kind.clone()));
            game_state.set(GameState::PlayerInput).expect("Switching states failed");
        }
    }
}

pub fn interact(
    mut ev_command: EventReader<CommandEvent>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for ev in ev_command.iter() {
        if let CommandType::Interact(action) = &ev.0 {
            ev_action.send(ActionEvent(action.clone()));
        }
    }
}

pub fn pick_item(
    mut commands: Commands,
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<&Parent>,
    mut ev_ui: EventWriter<ui::ReloadUIEvent>,
    mut res: ResMut<super::GameRes>
) {
    for ev in ev_command.iter() {
        if let CommandType::PickItem(entity) = &ev.0 {
            let player_entity = player_query.get_single().unwrap();
            let parent = item_query.get(*entity).unwrap();

            commands.entity(parent.get())
                .remove_children(&[*entity]);
            commands.entity(*entity)
                .remove::<SpriteSheetBundle>()
                .remove::<components::Piece>();
            commands.entity(player_entity)
                .push_children(&[*entity]);

            // remove action from the list and reload the UI
            res.input_commands.retain(|a| *a != ev.0);
            ev_ui.send(ui::ReloadUIEvent);
        }
    }
}

pub fn use_item(
    mut commands: Commands,
    mut ev_command: EventReader<CommandEvent>,
    mut ev_action: EventWriter<ActionEvent>,
    item_query: Query<&components::Manual>,
    mut ev_ui: EventWriter<ui::ReloadUIEvent>,
) {
    for ev in ev_command.iter() {
        if let CommandType::UseItem(entity) = &ev.0 {
            let manual = match item_query.get(*entity) {
                Ok(m) => m,
                _ => continue
            };

            ev_action.send(ActionEvent(manual.kind.clone()));
            ev_ui.send(ui::ReloadUIEvent);

            commands.entity(*entity)
                .despawn_recursive();
        }
    }
}