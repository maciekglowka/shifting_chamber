use bevy::prelude::*;
use serde::Deserialize;
use std::collections::VecDeque;

use crate::pieces::components::Health;
use crate::tiles::Tile;
use crate::ui::BubbleEvent;

mod units;

#[derive(Event)]
pub struct ActionEvent(pub Box<dyn Action>);
// // pub struct ActionEvent(pub ActionKind);

// #[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
// pub enum ActionKind {
//     Damage(Entity, DamageKind, u32),
//     Heal(Entity, u32),
//     IncreaseHP(Entity, u32)
// }

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_systems(Update, handle_event.run_if(on_event::<ActionEvent>()));
            // .add_systems(
            //     Update,
            //     (
            //         units::receive_damage,
            //         units::heal,
            //         units::increase_hp,
            //     )
            // );
    }
}

fn handle_event(
    world: &mut World
) {
    let events = if let Some(mut res) = world.get_resource_mut::<Events<ActionEvent>>() {
        res.drain().collect::<Vec<_>>()
    } else { return };
    for ev in events {
        ev.0.execute(world);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DamageKind {
    None,
    Hit,
    Fire
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World);
}

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