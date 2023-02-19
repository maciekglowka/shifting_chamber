use bevy::prelude::*;
use serde::Deserialize;

use crate::vectors::Vector2Int;

mod pieces;
mod units;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ActionKind {
    Damage(Entity, DamageKind, u32),
    SpawnPiece(Vector2Int, String)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_system(units::receive_damage)
                    .with_system(pieces::spawn_piece)
            );
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    None,
    Hit,
    Fire
}
