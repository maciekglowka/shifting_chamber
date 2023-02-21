use bevy::prelude::*;

use crate::pieces::components::{
    Health
};
use crate::ui::BubbleEvent;

use super::{ActionEvent, ActionKind};

pub fn receive_damage(
    mut health_query: Query<&mut Health>,
    mut ev_action: EventReader<ActionEvent>,
    // mut ev_bubble: EventWriter<BubbleEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Damage(entity, _kind, value) = ev.0 {
            if let Ok(mut health) = health_query.get_mut(entity) {
                health.sub(value);
                info!("{:?} got {} dmg", entity, value);
                // ev_bubble.send(BubbleEvent(transform.translation(), format!("-{}", dmg)));
            }
        }
    }
}
