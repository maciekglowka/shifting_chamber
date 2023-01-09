use bevy::prelude::*;

use crate::pieces::components::Piece;
use crate::player::Player;
use crate::states::GameState;
use crate::tiles;
use crate::pieces::components;

use super::{CommandEvent, CommandType, GameRes};


pub fn clear_actions(
    mut res: ResMut<GameRes>
) {
    res.input_actions.clear();
}

// all those systems are invoked by a direct player command
// they should end player_input state on success


pub fn shift_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Player>,
    unit_query: Query<&Parent, With<components::Unit>>,
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

pub fn pick_item(
    mut ev_command: EventReader<CommandEvent>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<&Parent>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::PickItem(entity) = ev.0 {
            let player_entity = player_query.get_single().unwrap();
            let parent = item_query.get(entity).unwrap();

            commands.entity(parent.get())
                .remove_children(&[entity]);
            commands.entity(entity)
                .remove::<SpriteSheetBundle>()
                .remove::<Piece>();
            commands.entity(player_entity)
                .push_children(&[entity]);

            game_state.set(GameState::TileShift).expect("Switching states failed");
        }
    }
}