use bevy::prelude::*;
use std::collections::HashMap;

use crate::globals::RESTART_PENALTY;
use crate::manager::GameRes;
use crate::pieces::components::Health;
use crate::player::{
    Player,
    upgrades::{UpgradeKind, TransformKind, get_all_transforms}
};
use crate::states::GameState;
use crate::tiles::transform::TileTransform;

use super::Action;


pub struct StartGameAction {
    pub level: i32
}
impl Action for StartGameAction {
    fn execute(&self, world: &mut World) {
        if let Some(mut res) = world.get_resource_mut::<GameRes>() {
            res.score = 0;
            res.level = self.level;
            res.max_ap = 1;
            res.tile_transforms = HashMap::from_iter(
                get_all_transforms().iter().map(|a| (*a, false))
            );
            // at the beggining only the default action is enabled
            res.tile_transforms.insert(TransformKind::default(), true);
            res.possible_upgrades = crate::player::upgrades::get_initial_upgrades();
        }

        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::MapInit);
        }
    }
}

pub struct RestartLevelAction;
impl Action for RestartLevelAction {
    fn execute(&self, world: &mut World) {
        let mut starting_hp = 0;
        if let Some(mut res) = world.get_resource_mut::<GameRes>() {
            res.score -= RESTART_PENALTY;
            starting_hp = res.level_starting_hp;
        }

        let mut health_query = world.query_filtered::<&mut Health, With<Player>>();
        if let Ok(mut health) = health_query.get_single_mut(world) {
            // restart players HP to level's initial
            health.value = starting_hp;
        }
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::MapInit);
        }
    }
}

pub struct RestartGameAction;
impl Action for RestartGameAction {
    fn execute(&self, world: &mut World) {
        if let Some(mut state) = world.get_resource_mut::<NextState<GameState>>() {
            state.set(GameState::MainMenu);
        }
    }
}