use bevy::prelude::*;
use std::collections::{HashSet, VecDeque};

use crate::actions::{ActionEvent, ActionKind};
use crate::manager::{CommandEvent, CommandType};
use crate::player::Player;
use crate::tiles::{Tile, TileRes};
use crate::vectors::{Vector2Int, ORTHO_DIRECTIONS};

use super::super::{
    components::{
        Damage,
        Health,
        Occupier,
        Range,
        Walking
    },
    PieceRes
};

pub fn plan_moves(
    mut walking_query: Query<(Entity, &mut Walking, &Parent)>,
    obstacle_query: Query<(&Parent, Option<&Occupier>, Option<&Damage>, Option<&Range>), Without<Player>>,
    player_query: Query<(Entity, &Parent), With<Player>>,
    tile_query: Query<&Tile>,
    tile_res: Res<TileRes>,
    mut piece_res: ResMut<PieceRes>
) {
    let mut avoid = HashSet::new();
    for (parent, occupier, damage, range) in obstacle_query.iter() {
        if occupier.is_none() && damage.is_none() { continue };
        let Ok(tile) = tile_query.get(parent.get()) else { continue };
        if occupier.is_some() { avoid.insert(tile.v); }
        if damage.is_some() {
            let Some(range) = range else { continue };
            for v in range.fields.iter() {
                avoid.insert(tile.v + *v);
            }
        }
    }

    let mut queue = VecDeque::new();
    let Ok((player_entity, player_parent)) = player_query.get_single() else { return };
    let player_v = match tile_query.get(player_parent.get()) {
        Ok(t) => t.v,
        _ => return
    };
    for (entity, mut walking, parent) in walking_query.iter_mut() {
        let mut possible = Vec::new();
        let Ok(tile) = tile_query.get(parent.get()) else { continue };
        for dir in ORTHO_DIRECTIONS {
            let v = tile.v + dir;
            if avoid.contains(&v) { continue };
            if !tile_res.tiles.contains_key(&v) { continue };

            let rank = player_v.manhattan(v);
            possible.push((rank, dir));

        }
        possible.sort_by(|a, b| a.0.cmp(&b.0));
        walking.planned_move = match possible.iter().next() {
            Some(a) => {
                queue.push_back(entity);
                if tile.v + a.1 != player_v { avoid.insert(tile.v + a.1); }
                Some(a.1)
            },
            None => None
        }
    }
    piece_res.walking_queue = queue;
}

pub fn move_walking(
    mut commands: Commands,
    walking_query: Query<(&Walking, &Parent)>,
    tile_query: Query<&Tile>,
    tile_res: Res<TileRes>,
    mut piece_res: ResMut<PieceRes>,
    mut ev_command: EventWriter<CommandEvent>
) {
    piece_res.walkign_active = None;
    let Some(entity) = piece_res.walking_queue.pop_front() else {
        ev_command.send(CommandEvent(CommandType::TurnEnd));
        return;
    };
    let Ok((walking, parent)) = walking_query.get(entity) else { return };
    let Some(dir) = walking.planned_move else { return };
    let Ok(tile) = tile_query.get(parent.get()) else { return };
    let v = tile.v + dir;
    if !tile_res.tiles.contains_key(&v) {
        return;
    }

    change_parent_tile(&mut commands, entity, parent, v, tile_res.as_ref());
    piece_res.walkign_active = Some(entity);
}

pub fn walk_back(
    mut commands: Commands,
    occupier_query: Query<(Entity, &Parent), With<Occupier>>,
    walking_query: Query<&Walking>,
    tile_query: Query<&Tile>,
    piece_res: Res<PieceRes>,
    tile_res: Res<TileRes>,
) {
    let Some(entity) = piece_res.walkign_active else { return };
    // if the occupier query won't have result it means our walker
    // is not an occupier and should not be checked
    let Ok((_, parent)) = occupier_query.get(entity) else { return };
    for (other, other_parent) in occupier_query.iter() {
        if parent.get() != other_parent.get() || other == entity { continue };
        // confilct -> move back to the previous position
        let Ok(walking) = walking_query.get(entity) else { continue };
        let Ok(tile) = tile_query.get(parent.get()) else { continue };
        let Some(dir) = walking.planned_move else { continue };
        let v = tile.v - dir;
        change_parent_tile(&mut commands, entity, parent, v, tile_res.as_ref());
    }
}

pub fn walk_damage(
    damage_query: Query<(&Damage, &Parent)>,
    health_query: Query<(Entity, &Parent), With<Health>>,
    piece_res: Res<PieceRes>,
    mut ev_action: EventWriter<ActionEvent>
) {
    let Some(entity) = piece_res.walkign_active else { return };
    let Ok((damage, parent)) = damage_query.get(entity) else { return };
    for (other, other_parent) in health_query.iter() {
        if parent.get() != other_parent.get() || other == entity { continue };
        ev_action.send(ActionEvent(
            ActionKind::Damage(other, damage.kind, damage.value)
        ));
    }
}

fn change_parent_tile(
    commands: &mut Commands,
    entity: Entity,
    current_parent: &Parent,
    new_v: Vector2Int,
    tile_res: &TileRes,
) {
    let Some(new_tile_entity) = tile_res.tiles.get(&new_v) else { return };
    commands.entity(current_parent.get()).remove_children(&[entity]);
    commands.entity(*new_tile_entity).add_child(entity);
}