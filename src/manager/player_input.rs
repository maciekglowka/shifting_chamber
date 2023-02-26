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

pub fn transform_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Parent, With<Player>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>
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
                game_state.set(GameState::TileShift).expect("Switching states failed");
            }
        }
    }
}

// pub fn switch_tiles(
//     mut ev_command: EventReader<CommandEvent>,
//     player_query: Query<&Parent, With<Player>>,
//     mut tile_query: Query<&mut tiles::Tile>,
//     mut tile_res: ResMut<tiles::TileRes>,
//     mut game_state: ResMut<State<GameState>>
// ) {
//     for ev in ev_command.iter() {
//         if let CommandType::SwitchTiles(dir) = ev.0 {
//             let player_v = match player_query.get_single() {
//                 Err(_) => continue,
//                 Ok(parent) => {
//                     let Ok(tile) = tile_query.get(parent.get()) else { continue };
//                     tile.v
//                 }
//             };

//             let transform = tiles::transform::TileTransform::Switch(dir);
//             if tiles::transform::can_transform(transform, player_v, tile_res.as_ref()) {
//                 tiles::transform::execute(
//                     transform,
//                     player_v,
//                     &mut tile_query,
//                     tile_res.as_mut()
//                 );
//                 game_state.set(GameState::TileShift).expect("Switching states failed");
//             }
//         }
//     }
// }


// pub fn shift_tiles(
//     mut ev_command: EventReader<CommandEvent>,
//     player_query: Query<&Parent, With<Player>>,
//     mut tile_query: Query<&mut tiles::Tile>,
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
//             // tiles::shift_tiles(player_v, dir, &mut tile_query, tile_res.as_mut());
//             tiles::transform::execute(
//                 tiles::transform::TileTransform::Shift(dir),
//                 player_v,
//                 &mut tile_query,
//                 tile_res.as_mut()
//             );
//             game_state.set(GameState::TileShift).expect("Switching states failed");
//         }
//     }
// }
