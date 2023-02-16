use bevy::prelude::*;
use serde::Deserialize;

mod units;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ActionKind {
    Damage(Entity, DamageKind, u32)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_system(units::receive_damage)
            );
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    None,
    Hit,
    Fire
}
