use bevy::prelude::*;

use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

mod action;
mod map_init;
mod player_input;

#[derive(Clone, Copy, Debug)]
pub enum CommandType {
    MapShift(Vector2Int, Vector2Int),
    AnimationEnd
}

pub struct CommandEvent(pub CommandType);


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::MapInit)
                    .with_system(map_init::start_game)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PlayerInput)
                    .with_system(player_input::shift_tiles)
            )
            .add_system(update_state);
    }
}

pub fn update_state(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>,
    mut player_query: Query<&mut Player>
) {
    for ev in ev_command.iter() {
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.current() {
                GameState::TileShift => {
                    game_state.set(GameState::Action);
                },
                GameState::Action => {
                    if let Ok(mut player) = player_query.get_single_mut() {
                        if player.is_descending {
                            player.is_descending = false;
                            game_state.set(GameState::MapInit);
                        } else {
                            game_state.set(GameState::PlayerInput);
                        }
                    }
                },
                _ => ()
            }
        }
    }
}
