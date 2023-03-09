use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind, DamageKind};
use crate::tiles::{Tile, TileRes};
use crate::vectors::OMNI_DIRECTIONS;

use super::super::{PieceEvent, PieceEventKind, PieceRes};
use super::super::components::{
    Damage,
    Explosive,
    Health,
    Projectile,
    Range
};

pub fn launch_projectiles(
    mut commands: Commands,
    range_query: Query<(&Damage, &Range, &Parent)>,
    health_query: Query<Entity, With<Health>>,
    tile_query: Query<(&Tile, &Children)>,
    tile_res: Res<TileRes>,
    piece_res: Res<PieceRes>
) {
    let Some(entity) = piece_res.action_queue.get(0) else { return };
    let Ok((damage, range, parent)) = range_query.get(*entity) else { return };

    let Ok(parent_tile) = tile_query.get(parent.get()) else { return };
    let affected_tiles: Vec<_> = range.fields.iter()
        .flat_map(|v| tile_res.tiles.get(&(parent_tile.0.v + *v)))
        .flat_map(|e| tile_query.get(*e))
        .collect();

    for tile in affected_tiles {
        if !tile.1.iter()
            .any(|a| health_query.get(*a).is_ok())
            { continue };
        commands.spawn((
            Projectile { 
                source: parent_tile.0.v,
                target: tile.0.v
            },
            damage.clone()
        ));
    }
}

pub fn explode_projectiles(
    mut commands: Commands,
    mut removed: RemovedComponents<Explosive>,
    mut ev_piece: EventReader<PieceEvent>,
    tile_query: Query<&Tile>,
    tile_res: Res<TileRes>,
    mut piece_res: ResMut<PieceRes>
) {
    for ev in ev_piece.iter() {
        if let PieceEventKind::Kill(entity, v) = ev.0 {
            if !removed.iter().any(|a| a == entity) { continue }

            let affected_tiles: Vec<_> = OMNI_DIRECTIONS.iter()
                .flat_map(|d| tile_res.tiles.get(&(v + *d)))
                .flat_map(|e| tile_query.get(*e))
                .collect();
            for tile in affected_tiles {
                commands.spawn((
                    Projectile { 
                        source: v,
                        target: tile.v
                    },
                    Damage { kind: DamageKind::Hit, value: 1}
                ));
            }
            // add dummy entity to the second spot of the queue
            // to make room explosion projectiles
            // current first will be popped on the beggining of next NPCAction phase
            let first = piece_res.action_queue.pop_front().unwrap();
            piece_res.action_queue.push_front(entity);
            piece_res.action_queue.push_front(first);
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
