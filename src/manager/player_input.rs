use bevy::prelude::*;

use crate::states::GameState;
use crate::tiles;
use super::{CommandEvent, CommandType};


pub fn shift_tiles(
    mut ev_command: EventReader<CommandEvent>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::MapShift(dir) = ev.0 {
            tiles::shift_tiles(-1 * dir, &mut tile_query, tile_res.as_mut());
            game_state.set(GameState::TileShift).expect("Switching states failed");
        }
    }
}