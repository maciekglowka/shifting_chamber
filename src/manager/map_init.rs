use bevy::prelude::*;

use crate::states::GameState;

pub fn start_game(
    mut game_state: ResMut<State<GameState>>
) {
    game_state.set(GameState::PlayerInput).expect("Switching states failed");
}