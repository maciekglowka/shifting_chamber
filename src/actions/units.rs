use bevy::prelude::*;

use crate::pieces::components::Unit;
use crate::player::Player;

use super::{ActionEvent, ActionKind};

pub fn receive_damage(
    mut unit_query: Query<&mut Unit>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Damage(entity, value) = ev.0 {
            if let Ok(mut unit) = unit_query.get_mut(entity) {
                unit.hp = unit.hp.saturating_sub(value);
            }
        }
    }
}
