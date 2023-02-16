use bevy::prelude::*;

use crate::pieces::components::{
    Health
};
use crate::ui::BubbleEvent;

use super::{ActionEvent, ActionKind};

pub fn receive_damage(
    mut health_query: Query<&mut Health>,
    mut ev_action: EventReader<ActionEvent>,
    mut ev_bubble: EventWriter<BubbleEvent>
) {
    for ev in ev_action.iter() {
        // if let ActionKind::Damage(entity, kind, value) = ev.0 {
        //     let mut dmg = value;
            
        //     if let Ok((mut unit, transform, children)) = unit_query.get_mut(entity) {
        //         for child in children.iter().flat_map(|v| *v) {
        //             if let Ok(p) = protect_query.get(*child) {
        //                 if p.kind == kind {
        //                     dmg = dmg.saturating_sub(p.value);
        //                 }
        //             }
        //         }
        //         unit.sub_hp(dmg);
        //         ev_bubble.send(BubbleEvent(transform.translation(), format!("-{}", dmg)));
        //     }
        // }
    }
}
