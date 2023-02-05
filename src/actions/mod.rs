use bevy::prelude::*;
use serde::Deserialize;

use crate::states::GameState;

mod pieces;
mod player;
mod units;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ActionKind {
    ApplyEffect(String),
    Damage(Entity, DamageKind, u32),
    Descend,
    Heal(u32),
    HealPoison,
    Poison(Entity, u32),
    Score(u32),
    SpawnPiece(Entity, String),
    StatUpgrade(StatKind, u32),
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_system(units::receive_damage)
                    .with_system(units::get_poisoned)
                    .with_system(pieces::spawn_piece)
                    .with_system(player::apply_effect)
                    .with_system(player::descend)
                    .with_system(player::heal)
                    .with_system(player::heal_poison)
                    .with_system(player::score)
                    .with_system(player::stat_upgrade)
                    .label("action")
            );
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum StatKind {
    HP,
    ST
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    None,
    Hit,
    Fire
}
