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
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::PlayerWait = ev.0 {
            game_state.set(GameState::NPCMove).expect("Switching states failed");
        }
    }
}

pub fn punch(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Parent, With<Player>>,
    health_query: Query<&components::Health>,
    mut tile_query: Query<(&mut tiles::Tile, &Children)>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for ev in ev_command.iter() {
        if let CommandType::Punch(dir) = ev.0 {
            let player_v = match player_query.get_single() {
                Err(_) => continue,
                Ok(parent) => {
                    let Ok((tile, _)) = tile_query.get(parent.get()) else { continue };
                    tile.v
                }
            };
            let Some(target_tile) = tile_res.tiles.get(&(player_v + dir)) else { continue };
            let Ok((_, tile_children)) = tile_query.get(*target_tile) else { continue };
            for entity in tile_children.iter() {
                if let Ok(_) = health_query.get(*entity) {
                    ev_action.send(
                        ActionEvent(ActionKind::Damage(*entity, DamageKind::Hit, 1)
                    ))
                }
            }

            game_state.set(GameState::TileShift).expect("Switching states failed");
        }
    }
}

pub fn switch_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Parent, With<Player>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::SwitchTiles(dir) = ev.0 {
            let player_v = match player_query.get_single() {
                Err(_) => continue,
                Ok(parent) => {
                    let Ok(tile) = tile_query.get(parent.get()) else { continue };
                    tile.v
                }
            };
            if tiles::can_switch(player_v, dir, &tile_res) {
                tiles::switch_tiles(player_v, dir, &mut tile_query, tile_res.as_mut());
                game_state.set(GameState::TileShift).expect("Switching states failed");
            }
        }
    }
}

// pub fn shift_tiles(
//     mut commands: Commands,
//     mut ev_command: EventReader<CommandEvent>,
//     player_query: Query<&Parent, With<Player>>,
//     tile_query: Query<&tiles::Tile>,
//     tile_children: Query<&Children, With<tiles::Tile>>,
//     occupier_query: Query<&components::Occupier>,
//     mut tile_res: ResMut<tiles::TileRes>,
//     mut game_state: ResMut<State<GameState>>
// ) {
//     for ev in ev_command.iter() {
//         if let CommandType::ShiftTiles(dir) = ev.0 {
//             let player_v = match player_query.get_single() {
//                 Err(_) => continue,
//                 Ok(parent) => {
//                     let Ok(tile) = tile_query.get(parent.get()) else { continue };
//                     tile.v
//                 }
//             };
//             tiles::shift_tiles(&mut commands, player_v, dir, &tile_children, &occupier_query, tile_res.as_mut());
//             game_state.set(GameState::TileShift).expect("Switching states failed");
//         }
//     }
// }

pub fn shift_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Parent, With<Player>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::ShiftTiles(dir) = ev.0 {
            let player_v = match player_query.get_single() {
                Err(_) => continue,
                Ok(parent) => {
                    let Ok(tile) = tile_query.get(parent.get()) else { continue };
                    tile.v
                }
            };
            tiles::shift_tiles(player_v, dir, &mut tile_query, tile_res.as_mut());
            game_state.set(GameState::TileShift).expect("Switching states failed");
        }
    }
}
