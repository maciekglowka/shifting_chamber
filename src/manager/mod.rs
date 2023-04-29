use bevy::prelude::*;
use std::{
    cmp,
    collections::{HashMap, HashSet}
};

use crate::globals::{
    UPGRADE_EVERY_LEVELS,
    LEVEL_BONUS,
    RESTART_PENALTY
};
use crate::pieces::components::{Health, Walking};
use crate::player::{
    Player,
    upgrades::{UpgradeKind, TransformKind, get_all_transforms}
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
    Upgrade(UpgradeKind),
    Start,
    RestartGame,
    RestartLevel
}

pub struct CommandEvent(pub CommandType);

pub enum GameEventKind {
    ProjectileLaunch,
    TileTransformed,
    UnitAttack,
    WrongAction
}

pub struct GameEvent(pub GameEventKind);

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>()
            .add_event::<GameEvent>()
            .init_resource::<GameRes>()
            .add_system(start_game.in_set(OnUpdate(GameState::GameInit)))
            .add_system(start_map.in_set(OnUpdate(GameState::MapInit)))
            .add_system(map_end.in_set(OnUpdate(GameState::MapEnd)))
            .add_systems(
                (player_input::transform_tiles, player_input::wait)
                .in_set(OnUpdate(GameState::PlayerInput)))
            .add_system(player_input::upgrade.in_set(OnUpdate(GameState::Upgrade)))
            .add_system(update_state.in_base_set(CoreSet::Last));

    }
}

fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    mut res: ResMut<GameRes>
) {
    res.score = 0;
    res.level = 1;
    res.max_ap = 1;
    res.tile_transforms = HashMap::from_iter(
        get_all_transforms().iter().map(|a| (*a, false))
    );
    // at the beggining only the default action is enabled
    res.tile_transforms.insert(TransformKind::default(), true);
    // tests only
    // res.tile_transforms.insert(TransformKind::TileSwitch, true);
    res.possible_upgrades = crate::player::upgrades::get_initial_upgrades();
    next_state.set(GameState::MapInit);
}

fn start_map(
    mut next_state: ResMut<NextState<GameState>>,
    mut res: ResMut<GameRes>,
    health_query: Query<&Health, With<Player>>
) {
    res.ap = 0;
    if let Ok(health) = health_query.get_single() {
        res.level_starting_hp = health.value;
    }
    next_state.set(GameState::TurnStart);
}

fn map_end(
    mut next_state: ResMut<NextState<GameState>>,
    mut res: ResMut<GameRes>,
    data_assets: Res<crate::data::DataAssets>
) {
    res.level += 1;
    res.score += LEVEL_BONUS;
    if data_assets.level_list.len() == res.level as usize {
        next_state.set(GameState::GameWin);
        return;
    }
    if res.level % UPGRADE_EVERY_LEVELS == 1 {
        next_state.set(GameState::Upgrade);
    } else {
        next_state.set(GameState::MapInit);
    }
}

pub fn update_state(
    mut ev_command: EventReader<CommandEvent>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    // player_query: Query<&Player>,
    npc_query: Query<&Walking>,
    mut res: ResMut<GameRes>,
    mut health_query: Query<&mut Health, With<Player>>
) {
    for ev in ev_command.iter() {
        if let CommandType::TurnEnd = ev.0 {
            next_state.set(GameState::TurnEnd);
            break;
        }
        if let CommandType::Start = ev.0 {
            next_state.set(GameState::GameInit);
            break;
        }
        if let CommandType::RestartLevel = ev.0 {
            res.score -= RESTART_PENALTY;
            if let Ok(mut health) = health_query.get_single_mut() {
                // restart players HP to level's initial
                health.value = res.level_starting_hp;
            }
            next_state.set(GameState::MapInit);
            break;
        }
        if let CommandType::RestartGame = ev.0 {
            next_state.set(GameState::MainMenu);
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
                    let Ok(player_health) = health_query.get_single() else { break };
                    match player_health.value {
                        a if a == 0 => { next_state.set(GameState::GameOver) },
                        _ => {
                            if npc_query.iter().len() == 0 {
                                next_state.set(GameState::MapEnd);
                            } else {
                                next_state.set(GameState::TurnStart);
                            }          
                        },
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
    pub ap: u32,
    pub max_ap: u32,
    pub ap_stacking: bool,
    pub possible_upgrades: HashSet<UpgradeKind>,
    pub score: u32,
    pub level_starting_hp: u32,
    // actions with 'true' value are enabled for the player
    pub tile_transforms: HashMap<TransformKind, bool>
}