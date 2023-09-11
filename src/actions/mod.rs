use bevy::prelude::*;
use serde::Deserialize;

mod models_game_state;
mod models_unit;
mod units;

pub use models_unit::*;
pub use models_game_state::*;

#[derive(Event)]
pub struct ActionEvent(pub Box<dyn Action>);

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_systems(Update, handle_event.run_if(on_event::<ActionEvent>()));
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
