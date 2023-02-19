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

pub fn wait(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::PlayerWait = ev.0 {
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



// pub fn shift_tiles(
//     mut ev_command: EventReader<CommandEvent>,
//     player_query: Query<&Parent, With<Player>>,
//     occupier_query: Query<&Parent, With<components::Occupier>>,
//     mut tile_query: Query<&mut tiles::Tile>,
//     mut tile_res: ResMut<tiles::TileRes>,
//     mut game_state: ResMut<State<GameState>>,
//     mut ev_tile: EventWriter<tiles::TileSwapEvent>
// ) {
//     // TODO refactor the whole process
//     for ev in ev_command.iter() {
//         if let CommandType::MapShift(v0, v1) = ev.0 {
//             if v0.manhattan(v1) != 1 { continue; }

//             let player_v = match player_query.get_single() {
//                 Err(_) => continue,
//                 Ok(parent) => {
//                     let Ok(tile) = tile_query.get(parent.get()) else { continue };
//                     tile.v
//                 }
//             };

//             if tiles::can_shift(v0, v1-v0, player_v, &occupier_query, &tile_res) {
//                 tiles::shift_tiles(v0, v1-v0, &mut tile_query, tile_res.as_mut(), &mut ev_tile);
//                 game_state.set(GameState::TileShift).expect("Switching states failed");
//             }
//         }
//     }
// }
