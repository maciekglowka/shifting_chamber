use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::data::DataAssets;
use crate::player::Player;
use crate::tiles::Tile;
use crate::ui::BubbleEvent;
use crate::vectors::Vector2Int;

use super::super::{
    components::{
        get_effective_dmg,
        get_poisonous,
        Damage,
        Poisoned,
        Poisonous,
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

pub fn apply_poison(
    mut unit_query: Query<(&mut Unit, &Poisoned, &GlobalTransform)>,
    mut ev_bubble: EventWriter<BubbleEvent>
) {
    for (mut unit, poison, transform) in unit_query.iter_mut() {
        unit.sub_hp(poison.value);
        ev_bubble.send(BubbleEvent(transform.translation(), format!("-{}", poison.value)));
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

pub fn check_poisoning(
    poisonous_query: Query<&Poisonous>,
    player_query: Query<(Entity, &Player, Option<&Children>)>,
    unit_query: Query<(Entity, &Unit, &Parent, Option<&Children>), Without<Player>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    let (player_entity, player, player_children) = player_query.get_single().unwrap();
    for (npc_entity, _, _, npc_children) in get_close_units(player.v, &unit_query, &tile_query) {
        let npc_poison = get_poisonous(npc_entity, &poisonous_query, npc_children);
        let player_poison = get_poisonous(player_entity, &poisonous_query, player_children);

        if let Some(p) = npc_poison {
            ev_action.send(ActionEvent(ActionKind::Poison(player_entity, p)));
        }
        if let Some(p) = player_poison {
            ev_action.send(ActionEvent(ActionKind::Poison(npc_entity, p)));
        }
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