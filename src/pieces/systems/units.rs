use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::{ActionEvent, ActionKind};
use crate::data::DataAssets;
use crate::manager::{CommandEvent, CommandType};
use crate::player::Player;
use crate::tiles::{Tile, TileRes};
use crate::ui::BubbleEvent;
use crate::vectors::{Vector2Int, ORTHO_DIRECTIONS};

use super::super::{
    components::{
        Damage,
        Health,
        Walking
    },
    PieceRes
};

pub fn kill_units(
    mut commands: Commands,
    health_query: Query<(Entity, Option<&Name>, &Health)>,
    data_assets: Res<DataAssets>,
    mut ev_action: EventWriter<ActionEvent>
) {
    for (entity, name, health) in health_query.iter() {
        if health.value > 0 { continue; }
        commands.entity(entity).despawn_recursive();
    }
}

pub fn plan_moves(
    mut walking_query: Query<(Entity, &mut Walking, &Parent)>,
    player_query: Query<&Parent, With<Player>>,
    tile_query: Query<&Tile>,
    tile_res: Res<TileRes>,
    mut piece_res: ResMut<PieceRes>
) {
    let mut queue = VecDeque::new();
    let Ok(player_parent) = player_query.get_single() else { return };
    let player_v = match tile_query.get(player_parent.get()) {
        Ok(t) => t.v,
        _ => return
    };
    for (entity, mut walking, parent) in walking_query.iter_mut() {
        let mut possible = Vec::new();
        let Ok(tile) = tile_query.get(parent.get()) else { continue };
        for dir in ORTHO_DIRECTIONS {
            let v = tile.v + dir;
            if !tile_res.tiles.contains_key(&v) { continue }
            let rank = player_v.manhattan(v);
            possible.push((rank, dir));
        }
        possible.sort_by(|a, b| a.0.cmp(&b.0));
        walking.planned_move = match possible.iter().next() {
            Some(a) => {
                queue.push_back(entity);
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
    player_query: Query<(Entity, &Player)>,
    tile_query: Query<&Tile>,
    tile_res: Res<TileRes>,
    mut piece_res: ResMut<PieceRes>,
    mut ev_command: EventWriter<CommandEvent>
) {
    let Some(entity) = piece_res.walking_queue.pop_front() else {
        ev_command.send(CommandEvent(CommandType::TurnEnd));
        return;
    };
    let Ok((walking, parent)) = walking_query.get(entity) else { return };
    let Some(dir) = walking.planned_move else { return };
    let Ok(tile) = tile_query.get(parent.get()) else { return };
    let v = tile.v + dir;

    let Some(new_tile_entity) = tile_res.tiles.get(&v) else { return };
    commands.entity(parent.get()).remove_children(&[entity]);
    commands.entity(*new_tile_entity).add_child(entity);
}
