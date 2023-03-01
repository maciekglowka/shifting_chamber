use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::tiles::{Tile, TileRes};

use super::super::components::{
    Damage,
    Health,
    Projectile,
    Range
};

pub fn launch_projectiles(
    mut commands: Commands,
    damage_query: Query<(&Damage, &Range, &Parent)>,
    tile_query: Query<&Tile>,
    tile_res: Res<TileRes>
) {
    for (damage, range, parent) in damage_query.iter() {
        let Ok(parent_tile) = tile_query.get(parent.get()) else { continue };
        let affected_tiles: Vec<_> = range.fields.iter()
            .flat_map(|v| tile_res.tiles.get(&(parent_tile.v + *v)))
            .flat_map(|e| tile_query.get(*e))
            .collect();

        for tile in affected_tiles {
            commands.spawn((
                Projectile { 
                    source: parent_tile.v,
                    target: tile.v
                },
                damage.clone()
            ));
        }
    }
}

pub fn hit_projectiles(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile, &Damage)>,
    health_query: Query<Entity, With<Health>>,
    tile_query: Query<&Children, With<Tile>>,
    tile_res: Res<TileRes>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (entity, projectile, damage) in projectile_query.iter() {
        // despawn first - to be sure the projectile won't live to the next turn
        // even if the tile is not found or has no children
        commands.entity(entity).despawn_recursive();
        let Some(tile_entity) = tile_res.tiles.get(&projectile.target) else { continue };
        let Ok(tile_children) = tile_query.get(*tile_entity) else { continue };

        for child in tile_children {
            if let Ok(health_entity) = health_query.get(*child) {
                ev_action.send(ActionEvent(
                    ActionKind::Damage(health_entity, damage.kind, damage.value)
                ));
            }
        }
    }
}
