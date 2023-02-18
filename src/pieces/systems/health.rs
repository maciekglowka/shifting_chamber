use bevy::prelude::*;


use super::super::components::Health;

pub fn kill_units(
    mut commands: Commands,
    health_query: Query<(Entity, &Health)>,
) {
    for (entity, health) in health_query.iter() {
        if health.value > 0 { continue; }
        commands.entity(entity).despawn_recursive();
    }
}