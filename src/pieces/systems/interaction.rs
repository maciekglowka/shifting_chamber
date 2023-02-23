use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::tiles::{Tile, TileRes};

use super::super::components::{
    Damage,
    Health,
    Range
};

pub fn interaction_damage(
    health_query: Query<Entity, With<Health>>,
    damage_query: Query<(&Damage, &Range, &Parent)>,
    tile_query: Query<(&Tile, &Children)>,
    tile_res: Res<TileRes>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (damage, range, parent) in damage_query.iter() {
        let Ok(parent_tile) = tile_query.get(parent.get()) else { continue };
        let affected_tiles: Vec<_> = range.fields.iter()
            .flat_map(|v| tile_res.tiles.get(&(parent_tile.0.v + *v)))
            .flat_map(|e| tile_query.get(*e))
            .collect();

        for (_, children) in affected_tiles {
            for child in children {
                if let Ok(entity) = health_query.get(*child) {
                    ev_action.send(ActionEvent(
                        ActionKind::Damage(entity, damage.kind, damage.value)
                    ));
                }
            }
        }
    }
}
