use bevy::prelude::*;

use crate::actions::ActionKind;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

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
            .init_resource::<GameRes>()
            .add_system_set(
                SystemSet::on_update(GameState::MapInit)
                    .with_system(map_init::start_map)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PlayerInput)
                    .with_system(player_input::shift_tiles)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerInput)
                    .with_system(player_input::clear_actions)
            )
            .add_system(update_state.after("action"));
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

#[derive(Default, Resource)]
pub struct GameRes {
    pub level: u32,
    pub score: u32,
    pub input_actions: Vec<ActionKind>
}