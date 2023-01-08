use bevy::prelude::*;

use crate::states::GameState;

pub fn start_map(
    mut game_state: ResMut<State<GameState>>,
    mut res: ResMut<super::GameRes>
) {
    res.level += 1;
    game_state.set(GameState::PlayerInput).expect("Switching states failed");
}