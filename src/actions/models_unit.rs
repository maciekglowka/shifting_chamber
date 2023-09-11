use bevy::prelude::*;

use crate::pieces::components::Health;
use crate::tiles::Tile;
use crate::ui::BubbleEvent;

use super::{Action, DamageKind};

pub struct DamageAction {
    pub entity: Entity,
    pub kind: DamageKind,
    pub value: u32
}
impl Action for DamageAction {
    fn execute(&self, world: &mut World) {
        if let Some(mut health) = world.get_mut::<Health>(self.entity) {
            health.sub(self.value);
        };
        let Some(parent) = world.get::<Parent>(self.entity) else { return };
        if let Some(tile) = world.get::<Tile>(parent.get()) {
            world.send_event(BubbleEvent(tile.v, format!("-{}", self.value)));
        }
    }
}

pub struct HealAction {
    pub entity: Entity,
    pub value: u32
}
impl Action for HealAction {
    fn execute(&self, world: &mut World) {
        if let Some(mut health) = world.get_mut::<Health>(self.entity) {
            health.add(self.value);
        };
    }
}

pub struct IncreaseHPAction {
    pub entity: Entity,
    pub value: u32
}
impl Action for IncreaseHPAction {
    fn execute(&self, world: &mut World) {
        if let Some(mut health) = world.get_mut::<Health>(self.entity) {
            health.max += self.value;
            health.add(self.value);
        };
    }
}