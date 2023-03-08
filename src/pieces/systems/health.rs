use bevy::prelude::*;


use super::super::components::Health;

pub fn kill_units(
    mut commands: Commands,
    health_query: Query<(Entity, &Health, Option<&Parent>)>,
) {
    for (entity, health, parent) in health_query.iter() {
        if health.value > 0 { continue; }
        if let Some(parent) = parent {
            commands.entity(parent.get()).remove_children(&[entity]);
        }
        commands.entity(entity).despawn_recursive();
    }
}