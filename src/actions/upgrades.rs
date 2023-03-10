use bevy::prelude::*;

use crate::pieces::components::{
    Health
};
use crate::player::Player;

use super::{ActionEvent, ActionKind};

pub fn heal_player(
    mut health_query: Query<&mut Health, With<Player>>,
    mut ev_action: EventReader<ActionEvent>,
) {
    for ev in ev_action.iter() {
        if let ActionKind::HealPlayer(value) = ev.0 {
            let Ok(mut health) = health_query.get_single_mut() else { continue };
            health.add(value);
        }
    }
}
