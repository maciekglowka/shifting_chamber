use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind, StatKind};
use crate::data::DataAssets;
use crate::player::Player;
use crate::tiles::Tile;

use super::super::{
    components::{
        Damage,
        Unit
    },
    renderer,
    spawn_piece_at_entity
};

pub fn check_fights(
    player_query: Query<(Entity, &Damage, &Player, &Unit)>,
    npc_query: Query<(Entity, &Damage, &Parent, &Unit), Without<Player>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (npc_entity, npc_damage, parent, npc_unit) in npc_query.iter() {
        let (player_entity, player_damage, player, player_unit) = player_query.get_single().unwrap();

        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v.manhattan(player.v) > 1 { continue; }

        let npc_dmg = npc_damage.value + npc_unit.stats.get(&StatKind::ST).unwrap_or(&0);
        let player_dmg = player_damage.value + player_unit.stats.get(&StatKind::ST).unwrap_or(&0);

        ev_action.send(ActionEvent(ActionKind::Damage(player_entity, npc_damage.kind, npc_dmg)));
        ev_action.send(ActionEvent(ActionKind::Damage(npc_entity, player_damage.kind, player_dmg)));
    }
}

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