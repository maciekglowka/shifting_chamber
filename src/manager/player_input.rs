use bevy::prelude::*;
use std::cmp;

use crate::actions::{ActionEvent, ActionKind, DamageKind};
use crate::pieces::components;
use crate::player::Player;
use crate::states::GameState;
use crate::tiles;
use crate::ui;

use super::{CommandEvent, CommandType, GameRes};

// all those systems are invoked by a direct player command
// they should end player_input state on success

pub fn wait(
    mut ev_command: EventReader<CommandEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::PlayerWait = ev.0 {
            next_state.set(GameState::NPCAction);
        }
    }
}

pub fn transform_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Parent, With<Player>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut next_state: ResMut<NextState<crate::states::GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::TransformTiles(transform) = ev.0 {
            let player_v = match player_query.get_single() {
                Err(_) => continue,
                Ok(parent) => {
                    let Ok(tile) = tile_query.get(parent.get()) else { continue };
                    tile.v
                }
            };
            if tiles::transform::can_transform(transform, player_v, tile_res.as_ref()) {
                tiles::transform::execute(
                    transform,
                    player_v,
                    &mut tile_query,
                    tile_res.as_mut()
                );
                next_state.set(GameState::TileShift);
            }
        }
    }
}

pub fn upgrade(
    mut ev_command: EventReader<CommandEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    mut ev_action: EventWriter<ActionEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::Upgrade(kind) = ev.0 {
            // ev_action.send(ActionEvent(action));
            next_state.set(GameState::MapInit);
        }
    }
}