use bevy::prelude::*;
use serde::Deserialize;

use crate::states::GameState;

mod pieces;
mod player;
mod units;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Debug, Deserialize)]
pub enum ActionKind {
    ApplyEffect(String),
    Damage(Entity, DamageKind, u32),
    Descend,
    Heal(u32),
    PickItem(Entity),
    Score(u32),
    SpawnPiece(Entity, String),
    StatUpgrade(StatKind, u32)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .init_resource::<ActionRes>()
            .add_system_set(
                SystemSet::new()
                    .label("action")
                    .with_system(units::receive_damage)
                    .with_system(pieces::spawn_piece)
                    .with_system(player::apply_effect)
                    .with_system(player::descend)
                    .with_system(player::heal)
                    .with_system(player::pick_item)
                    .with_system(player::score)
                    .with_system(player::stat_upgrade)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerInput)
                    .with_system(clear_input_actions)
            );
    }
}

pub fn clear_input_actions(
    mut res: ResMut<ActionRes>
) {
    res.input_actions.clear();
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum StatKind {
    HP,
    ST
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    Hit,
    Fire
}

#[derive(Default, Resource)]
pub struct ActionRes {
    pub input_actions: Vec<ActionKind>
}