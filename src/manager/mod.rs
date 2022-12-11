use bevy::prelude::*;

use crate::fixtures;
use crate::player;
use crate::states::GameState;
use crate::tiles;
use crate::vectors::Vector2Int;

mod action;
mod map_init;
mod player_input;

pub enum CommandType {
    MapShift(Vector2Int, Vector2Int),
    AnimationEnd,
}

pub struct CommandEvent(pub CommandType);


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>()
            .init_resource::<ManagerRes>()
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
            .add_system_set(
                SystemSet::on_enter(GameState::Action)
                    .with_system(handle_descend)
            )
            .add_system(update_state);
    }
}

pub fn update_state(
    mut ev_command: EventReader<CommandEvent>,
    mut game_state: ResMut<State<GameState>>,
    mut res: ResMut<ManagerRes>
) {
    for ev in ev_command.iter() {
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.current() {
                GameState::TileShift => {
                    game_state.set(GameState::Action);
                },
                GameState::Action => {
                    if res.is_descending {
                        res.is_descending = false;
                        game_state.set(GameState::MapInit);
                    } else {
                        game_state.set(GameState::PlayerInput);
                    }
                },
                _ => ()
            }
        }
    }
}

fn handle_descend(
    player_query: Query<&player::Player>,
    fixture_query: Query<&Parent, With<fixtures::Fixture>>,
    tile_query: Query<&tiles::Tile>,
    mut res: ResMut<ManagerRes>
) {
    for parent in fixture_query.iter() {
        let player = player_query.get_single().unwrap();

        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }
        res.is_descending = true;
    }
}

#[derive(Default, Resource)]
pub struct ManagerRes {
    pub is_descending: bool
}