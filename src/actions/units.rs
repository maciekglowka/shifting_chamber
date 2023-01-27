use bevy::prelude::*;

use crate::pieces::components::{Protect, Poisoned, Unit};
use crate::ui::BubbleEvent;

use super::{ActionEvent, ActionKind};

pub fn receive_damage(
    mut unit_query: Query<(&mut Unit, &GlobalTransform, Option<&Children>)>,
    mut ev_action: EventReader<ActionEvent>,
    protect_query: Query<&Protect>,
    mut ev_bubble: EventWriter<BubbleEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Damage(entity, kind, value) = ev.0 {
            let mut dmg = value;
            
            if let Ok((mut unit, transform, children)) = unit_query.get_mut(entity) {
                for child in children.iter().flat_map(|v| *v) {
                    if let Ok(p) = protect_query.get(*child) {
                        if p.kind == kind {
                            dmg = dmg.saturating_sub(p.value);
                        }
                    }
                }
                unit.sub_hp(dmg);
                ev_bubble.send(BubbleEvent(transform.translation(), format!("-{}", dmg)));
            }
        }
    }
}

pub fn get_poisoned(
    mut commands: Commands,
    mut unit_query: Query<Option<&mut Poisoned>, With<Unit>>,
    mut ev_action: EventReader<ActionEvent>,
) {
    for ev in ev_action.iter() {
        if let ActionKind::Poison(entity, value) = ev.0 {
            let poisoned = match unit_query.get_mut(entity) {
                Ok(p) => p,
                _ => continue
            };

            if let Some(mut poisoned) = poisoned {
                poisoned.value += value;
            } else {
                commands.entity(entity)
                    .insert(Poisoned { value });
            }
        }
    }
}
