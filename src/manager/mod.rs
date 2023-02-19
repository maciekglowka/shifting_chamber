use bevy::prelude::*;

use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

mod player_input;

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    // MapShift(Vector2Int, Vector2Int),
    SwitchTiles(Vector2Int),
    ShiftTiles(Vector2Int),
    PlayerWait,
    AnimationEnd,
    TurnEnd
}

pub struct CommandEvent(pub CommandType);


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>()
            .init_resource::<GameRes>()
            .add_system_set(
                SystemSet::on_update(GameState::GameInit)
                    .with_system(start_game)
            )
            .add_system_set(
                SystemSet::on_update(GameState::MapInit)
                    .with_system(start_map)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PlayerInput)
                    .with_system(player_input::switch_tiles)
                    .with_system(player_input::shift_tiles)
                    .with_system(player_input::wait)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerInput)
                    .with_system(clear_input_commands)
            )
            .add_system_set(
                SystemSet::on_update(GameState::NPCMove)
                    .with_system(turn_end)
            )
            .add_system(update_state);

    }
}

fn start_game(
    mut game_state: ResMut<State<GameState>>,
    mut res: ResMut<GameRes>
) {
    res.score = 0;
    res.level = 0;
    res.next_upgrade = 2;
    game_state.set(GameState::MapInit).expect("Switching states failed");
}

fn start_map(
    mut game_state: ResMut<State<GameState>>,
    mut res: ResMut<GameRes>
) {
    res.level += 1;
    game_state.set(GameState::PlayerInput).expect("Switching states failed");
}

pub fn turn_end(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    for ev in ev_command.iter() {
        if let CommandType::TurnEnd = ev.0 {
            game_state.set(GameState::TurnEnd);
        }
    }
}

pub fn update_state(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>,
    player_query: Query<&Player>,
    res: Res<GameRes>
) {
    for ev in ev_command.iter() {
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.current() {
                GameState::TileShift => {
                    game_state.set(GameState::NPCMove);
                },
                GameState::NPCMove => {
                    game_state.set(GameState::MoveResult);
                },
                GameState::MoveResult => {
                    game_state.set(GameState::NPCMove);
                },
                GameState::TurnEnd => {
                    match player_query.get_single() {
                        Ok(_) => {
                            if res.score >= res.next_upgrade {
                                game_state.set(GameState::Upgrade);
                                return;
                            } else {
                                game_state.set(GameState::PlayerInput);
                            }          
                        },
                        _ => { game_state.set(GameState::GameOver); },
                    }
                },
                _ => ()
            }
        }
    }
}

fn clear_input_commands(
    mut res: ResMut<GameRes>
) {
    res.input_commands.clear();
}

#[derive(Default, Resource)]
pub struct GameRes {
    pub level: u32,
    pub level_history: Vec<String>,
    pub score: u32,
    pub next_upgrade: u32,
    pub input_commands: Vec<CommandType>
}