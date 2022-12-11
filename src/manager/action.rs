use bevy::prelude::*;

use crate::units;

pub fn update_units(
    mut commands: Commands,
    unit_query: Query<(Entity, &units::Unit)>
) {
    for (entity, unit) in unit_query.iter() {
        if unit.hp > 0 { continue; }

        commands.entity(entity)
            .despawn_recursive();
    }
}