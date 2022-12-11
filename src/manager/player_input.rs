use bevy::prelude::*;

use crate::player;
use crate::states::GameState;
use crate::tiles;
use crate::units;

use super::{CommandEvent, CommandType};


pub fn shift_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&player::Player>,
    unit_query: Query<&Parent, With<units::Unit>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::MapShift(v0, v1) = ev.0 {
            if v0.manhattan(v1) != 1 { continue; }

            let player_v = player_query.get_single().unwrap().v;

            if tiles::can_shift(v0, v1-v0, player_v, &unit_query, &tile_res) {
                tiles::shift_tiles(v0, v1-v0, &mut tile_query, tile_res.as_mut());
                game_state.set(GameState::TileShift).expect("Switching states failed");
            }
        }
    }
}