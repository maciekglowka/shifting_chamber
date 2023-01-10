use bevy::prelude::*;
use serde::Deserialize;

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
    Score(u32),
    SpawnPiece(Entity, String)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system_set(
                SystemSet::new()
                    .label("action")
                    .with_system(units::receive_damage)
                    .with_system(pieces::spawn_piece)
                    .with_system(player::apply_effect)
                    .with_system(player::descend)
                    .with_system(player::heal)
                    .with_system(player::score)
            );
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    Hit,
    Fire
}