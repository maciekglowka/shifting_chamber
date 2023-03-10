use bevy::prelude::*;
use serde::Deserialize;

use crate::states::GameState;
use crate::vectors::Vector2Int;

mod pieces;
mod units;
mod upgrades;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum ActionKind {
    Damage(Entity, DamageKind, u32),
    HealPlayer(u32)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_systems((
                units::receive_damage,
                pieces::spawn_piece,
                upgrades::heal_player
            ));
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    None,
    Hit,
    Fire
}
