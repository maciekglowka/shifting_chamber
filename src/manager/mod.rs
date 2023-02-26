use bevy::prelude::*;
use std::cmp;

use crate::pieces::components::Walking;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;
use crate::tiles::transform::TileTransform;

mod player_input;

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    TransformTiles(TileTransform),
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
                    .with_system(player_input::transform_tiles)
                    .with_system(player_input::wait)
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
    res.ap = 0;
    game_state.set(GameState::TurnStart).expect("Switching states failed");
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
    npc_query: Query<&Walking>,
    mut res: ResMut<GameRes>
) {
    for ev in ev_command.iter() {
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.current() {
                GameState::TurnStart => {
                    res.ap = cmp::min(2, res.ap + 1);
                    game_state.set(GameState::PlayerInput);
                },
                GameState::TileShift => {
                    res.ap = res.ap.saturating_sub(1);
                    match res.ap {
                        0 => game_state.set(GameState::NPCMove),
                        _ => game_state.set(GameState::PlayerInput)
                    };
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
                            // if res.score >= res.next_upgrade {
                            //     game_state.set(GameState::Upgrade);
                            //     return;
                            if npc_query.iter().len() == 0 {
                                game_state.set(GameState::MapInit);
                            } else {
                                game_state.set(GameState::TurnStart);
                            }          
                        },
                        _ => { game_state.set(GameState::GameOver); },
                    }
                },
                _ => ()
            }
        }
        // change state only once
        break;
    }
}

#[derive(Default, Resource)]
pub struct GameRes {
    pub level: u32,
    pub level_history: Vec<String>,
    pub score: u32,
    pub next_upgrade: u32,
    pub ap: u32
}