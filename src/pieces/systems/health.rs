use bevy::prelude::*;

use crate::tiles::Tile;

use super::super::{PieceEvent, PieceEventKind};
use super::super::components::Health;
use crate::player::Player;

pub fn init_health(
    mut query: Query<&mut Health, Added<Health>>
) {
    for mut health in query.iter_mut() {
        health.value = health.max;
    }
}

pub fn kill_units(
    mut commands: Commands,
    health_query: Query<(Entity, &Health, Option<&Parent>), Without<Player>>,
    tile_query: Query<&Tile>,
    mut ev_piece: EventWriter<PieceEvent>
) {
    for (entity, health, parent) in health_query.iter() {
        if health.value > 0 { continue; }
        if let Some(parent) = parent {
            commands.entity(parent.get()).remove_children(&[entity]);
            if let Ok(tile) = tile_query.get(parent.get()) {
                ev_piece.send(
                    PieceEvent(PieceEventKind::Kill(entity, tile.v))
                )
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}