use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::data::DataAssets;
use crate::player::Player;
use crate::tiles::Tile;
use crate::vectors::Vector2Int;

use super::super::{
    components::{
        get_effective_dmg,
        Damage,
        Unit
    },
    renderer,
    spawn_piece_at_entity
};

pub fn kill_units(
    mut commands: Commands,
    unit_query: Query<(Entity, &Unit, Option<&Parent>)>,
    assets: Res<renderer::PieceAssets>,
    data_assets: Res<DataAssets>
) {
    for (entity, unit, parent) in unit_query.iter() {
        if unit.hp() > 0 { continue; }
        commands.entity(entity).despawn_recursive();
        if let Some(parent) = parent {
            spawn_piece_at_entity(
                &mut commands,
                "Coin".into(),
                parent.get(),
                assets.as_ref(),
                data_assets.as_ref()
            )
        }
    }
}

pub fn check_unit_damage(
    damage_query: Query<&Damage>,
    player_query: Query<(Entity, &Player, &Unit, Option<&Children>)>,
    unit_query: Query<(Entity, &Unit, &Parent, Option<&Children>), Without<Player>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    let (player_entity, player, player_unit, player_children) = player_query.get_single().unwrap();
    for (npc_entity, npc_unit, _, npc_children) in get_close_units(player.v, &unit_query, &tile_query) {
        let npc_dmg = get_effective_dmg(npc_entity, npc_unit, &damage_query, npc_children);
        let player_dmg = get_effective_dmg(player_entity, player_unit, &damage_query, player_children);

        ev_action.send(ActionEvent(ActionKind::Damage(player_entity, npc_dmg.0, npc_dmg.1)));
        ev_action.send(ActionEvent(ActionKind::Damage(npc_entity, player_dmg.0, player_dmg.1)));
    }
}

fn get_close_units<'a>(
    player_v: Vector2Int,
    unit_query: &'a Query<(Entity, &Unit, &Parent, Option<&Children>), Without<Player>>,
    tile_query: &'a Query<&Tile>
) -> Vec<(Entity, &'a Unit, &'a Parent, Option<&'a Children>)> {
    unit_query.iter()
        .filter(|(_, _, parent, _)| {
            tile_query.get(parent.get()).unwrap().v.manhattan(player_v) <= 1
        })
        .collect()
}