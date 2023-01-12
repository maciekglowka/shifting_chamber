use bevy::prelude::*;

use crate::pieces::components::{Protect, Unit};

use super::{ActionEvent, ActionKind};

pub fn receive_damage(
    mut unit_query: Query<(&mut Unit, Option<&Children>)>,
    mut ev_action: EventReader<ActionEvent>,
    protect_query: Query<&Protect>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Damage(entity, kind, value) = ev.0 {
            let mut dmg = value;
            
            if let Ok((mut unit, children)) = unit_query.get_mut(entity) {
                for child in children.iter().flat_map(|v| *v) {
                    if let Ok(p) = protect_query.get(*child) {
                        if p.kind == kind {
                            dmg = dmg.saturating_sub(p.value);
                        }
                    }
                }
                let hp = unit.hp().saturating_sub(dmg);
                unit.set_hp(hp);
            }
        }
    }
}
