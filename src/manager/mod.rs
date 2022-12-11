use bevy::prelude::*;

use crate::states::GameState;
use crate::vectors::Vector2Int;

mod action;
mod map_init;
mod player_input;

pub enum CommandType {
    MapShift(Vector2Int),
    AnimationEnd,
    // PickItem,
    // NextTurn
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
            .add_system_set(
                SystemSet::on_exit(GameState::Action)
                    .with_system(action::update_units)
            )
            .add_system(update_state);
    }
}

pub fn update_state(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.current() {
                GameState::TileShift => {
                    game_state.set(GameState::Action);
                },
                GameState::Action => {
                    game_state.set(GameState::PlayerInput);
                },
                _ => ()
            }
        }
    }
}