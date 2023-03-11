use bevy::prelude::*;
use std::{
    cmp,
    collections::HashSet
};

use crate::globals::UPGRADE_EVERY_LEVELS;
use crate::pieces::components::Walking;
use crate::player::{
    Player,
    upgrades::{UpgradeKind, TransformKind}
};
use crate::states::GameState;
use crate::tiles::transform::TileTransform;

mod player_input;

#[derive(Clone, Debug, PartialEq)]
pub enum CommandType {
    TransformTiles(TileTransform),
    PlayerWait,
    AnimationEnd,
    TurnEnd,
    Upgrade(UpgradeKind)
}

pub struct CommandEvent(pub CommandType);


pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>()
            .init_resource::<GameRes>()
            .add_system(start_game.in_set(OnUpdate(GameState::GameInit)))
            .add_system(start_map.in_set(OnUpdate(GameState::MapInit)))
            .add_system(map_end.in_set(OnUpdate(GameState::MapEnd)))
            .add_systems(
                (player_input::transform_tiles, player_input::wait)
                .in_set(OnUpdate(GameState::PlayerInput)))
            .add_system(player_input::upgrade.in_set(OnUpdate(GameState::Upgrade)))
            .add_system(update_state);

    }
}

fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut res: ResMut<GameRes>
) {
    res.level = 0;
    res.max_ap = 1;
    res.available_transforms = vec!(TransformKind::TileShift);
    res.possible_upgrades = crate::player::upgrades::get_initial_upgrades();
    next_state.set(GameState::MapInit);
}

fn start_map(
    mut next_state: ResMut<NextState<GameState>>,
    mut res: ResMut<GameRes>
) {
    res.level += 1;
    res.ap = 0;
    next_state.set(GameState::TurnStart);
}

fn map_end(
    mut next_state: ResMut<NextState<GameState>>,
    res: Res<GameRes>
) {
    if res.level % UPGRADE_EVERY_LEVELS == 0 {
        next_state.set(GameState::Upgrade);
    } else {
        next_state.set(GameState::MapInit);
    }
}

// pub fn turn_end(
//     mut ev_command: EventReader<CommandEvent>,
//     mut next_state: ResMut<NextState<GameState>>
// ) {
//     for ev in ev_command.iter() {
//         if let CommandType::TurnEnd = ev.0 {
//             next_state.set(GameState::TurnEnd);
//         }
//     }
// }

pub fn update_state(
    mut ev_command: EventReader<CommandEvent>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<&Player>,
    npc_query: Query<&Walking>,
    mut res: ResMut<GameRes>
) {
    for ev in ev_command.iter() {
        if let CommandType::TurnEnd = ev.0 {
            next_state.set(GameState::TurnEnd);
            break;
        }
        if let CommandType::AnimationEnd = ev.0 {
            match game_state.0 {
                GameState::TurnStart => {
                    if res.ap_stacking {
                        res.ap = cmp::min(res.max_ap, res.ap + 1);
                    } else {
                        res.ap = 1;
                    }
                    res.ap_stacking = true;
                    next_state.set(GameState::PlayerInput);
                },
                GameState::TileShift => {
                    res.ap = res.ap.saturating_sub(1);
                    res.ap_stacking = false;
                    match res.ap {
                        0 => next_state.set(GameState::NPCAction),
                        _ => next_state.set(GameState::PlayerInput)
                    };
                },
                GameState::NPCAction => {
                    next_state.set(GameState::NPCResult);
                },
                GameState::NPCResult => {
                    next_state.set(GameState::NPCAction);
                },
                GameState::TurnEnd => {
                    match player_query.get_single() {
                        Ok(_) => {
                            if npc_query.iter().len() == 0 {
                                next_state.set(GameState::MapEnd);
                            } else {
                                next_state.set(GameState::TurnStart);
                            }          
                        },
                        _ => { next_state.set(GameState::GameOver) },
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
    pub ap: u32,
    pub max_ap: u32,
    pub ap_stacking: bool,
    pub possible_upgrades: HashSet<UpgradeKind>,
    pub available_transforms: Vec<TransformKind>
}