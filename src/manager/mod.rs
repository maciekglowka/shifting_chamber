use bevy::prelude::*;

use crate::actions::ActionKind;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

mod player_input;

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    MapShift(Vector2Int, Vector2Int),
    AnimationEnd,
    NextLevel,
    Upgrade(ActionKind),
    Interact(ActionKind),
    PickItem(Entity),
    UseItem(Entity)
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
                    .with_system(player_input::shift_tiles)
                    .with_system(player_input::next_level)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerInput)
                    .with_system(clear_input_commands)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Upgrade)
                    .with_system(player_input::upgrade)
                    .before("action")
            )
            .add_system(update_state.after("action"))
            .add_system_set(
                SystemSet::new()
                    .with_system(player_input::use_item)
                    .with_system(player_input::interact)
                    .with_system(player_input::pick_item)
                    .label("input_command")
                    .before("action")
            );

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
                    game_state.set(GameState::ShiftResult);
                },
                GameState::ShiftResult => {
                    game_state.set(GameState::TurnEnd);
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