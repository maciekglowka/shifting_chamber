use bevy::prelude::*;
use serde::Deserialize;

mod player;
mod units;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ActionKind {
    Damage(Entity, DamageKind, u32),
    Descend,
    Heal(u32),
    Score(u32)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system_set(
                SystemSet::new()
                    .label("action")
                    .with_system(units::receive_damage)
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