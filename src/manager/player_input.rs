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
    // unit_query: Query<&Parent, With<components::Unit>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>,
    mut ev_tile: EventWriter<tiles::TileSwapEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::MapShift(v0, v1) = ev.0 {
            if v0.manhattan(v1) != 1 { continue; }

            // let player_v = player_query.get_single().unwrap().v;

            // if tiles::can_shift(v0, v1-v0, player_v, &unit_query, &tile_res) {
                tiles::shift_tiles(v0, v1-v0, &mut tile_query, tile_res.as_mut(), &mut ev_tile);
                game_state.set(GameState::TileShift).expect("Switching states failed");
            // }
        }
    }
}
