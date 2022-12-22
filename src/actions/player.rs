use bevy::prelude::*;
use std::cmp::min;

use crate::pieces::components::Unit;
use crate::player::Player;

use super::{ActionEvent, ActionKind};

pub fn heal(
    mut player_query: Query<&mut Unit, With<Player>>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Heal(val) = ev.0 {
            if let Ok(mut unit) = player_query.get_single_mut() {
                unit.hp = min(unit.max_hp, unit.hp + val);
            }
        }
    }
}

pub fn descend(
    mut player_query: Query<&mut Player>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Descend = ev.0 {
            if let Ok(mut player) = player_query.get_single_mut() {
                player.is_descending = true;
            }
        }
    }
}