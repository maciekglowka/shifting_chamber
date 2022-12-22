use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Damage,
    Unit
};

pub fn check_fights(
    player_query: Query<(Entity, &Damage, &Player)>,
    npc_query: Query<(Entity, &Damage, &Parent), Without<Player>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (npc_entity, npc_damage, parent) in npc_query.iter() {
        let (player_entity, player_damage, player) = player_query.get_single().unwrap();

        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v.manhattan(player.v) > 1 { continue; }

        ev_action.send(ActionEvent(ActionKind::Damage(player_entity, npc_damage.value)));
        ev_action.send(ActionEvent(ActionKind::Damage(npc_entity, player_damage.value)));
    }
}

pub fn kill_units(
    mut commands: Commands,
    unit_query: Query<(Entity, &Unit)>
) {
    for (entity, unit) in unit_query.iter() {
        if unit.hp > 0 { continue; }
        commands.entity(entity).despawn_recursive();
    }
}