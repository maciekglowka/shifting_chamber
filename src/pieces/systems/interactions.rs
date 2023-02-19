use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};

use super::super::components::{
    Damage,
    Health
};

pub fn interaction_damage(
    health_query: Query<(Entity, &Parent), With<Health>>,
    damage_query: Query<(Entity, &Damage, &Parent)>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (entity, parent) in health_query.iter() {
        for (dmg_entity, damage, dmg_parent) in damage_query.iter() {
            if parent != dmg_parent || entity == dmg_entity { continue };
            ev_action.send(ActionEvent(
                ActionKind::Damage(entity, damage.kind, damage.value)
            ));
        }
    }
}
