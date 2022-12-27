use bevy::prelude::*;
use serde::Deserialize;

mod player;
mod units;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ActionKind {
    Damage(Entity, u32),
    Descend,
    Heal(u32)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system(units::receive_damage)
            .add_system(player::descend)
            .add_system(player::heal);
    }
}
