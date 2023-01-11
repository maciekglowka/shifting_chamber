use bevy::prelude::*;

use crate::pieces::components::Unit;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

mod player_input;

#[derive(Clone, Debug)]
pub enum CommandType {
    MapShift(Vector2Int, Vector2Int),
    AnimationEnd,
    NextLevel
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
            .add_system(update_state.after("action"));
    }
}

fn start_game(
    mut game_state: ResMut<State<GameState>>,
    mut res: ResMut<GameRes>
) {
    res.score = 0;
    res.level = 0;
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
    mut player_query: Query<(&mut Player, &Unit)>
) {
    for ev in ev_command.iter() {
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.current() {
                GameState::TileShift => {
                    game_state.set(GameState::ShiftResult);
                },
                GameState::ShiftResult => {
                    if let Ok((mut player, unit)) = player_query.get_single_mut() {
                        if unit.hp == 0 {
                            game_state.set(GameState::GameOver);
                            return;
                        }                    
                        game_state.set(GameState::PlayerInput);
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
    pub score: u32}