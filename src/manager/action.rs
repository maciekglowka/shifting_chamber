use bevy::prelude::*;

use crate::states::GameState;
use crate::units;

use super::{CommandEvent, CommandType};

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
