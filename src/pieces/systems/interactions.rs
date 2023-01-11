use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind, ActionRes};
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Damage,
    Instant,
    Interactive,
    Piece,
};

pub fn check_instant(
    player_query: Query<&Player>,
    piece_query: Query<(&Parent, &Instant), With<Piece>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (parent, instant) in piece_query.iter() {
        if !is_player_tile(&player_query.get_single().unwrap(), parent, &tile_query) {
            continue;
        }
        ev_action.send(ActionEvent(instant.kind.clone()));
    }
}

pub fn check_interactions(
    player_query: Query<&Player>,
    piece_query: Query<(&Parent, &Interactive), With<Piece>>,
    tile_query: Query<&Tile>,
    mut action_res: ResMut<ActionRes>
) {
    for (parent, interactive) in piece_query.iter() {
        if !is_player_tile(&player_query.get_single().unwrap(), parent, &tile_query) {
            continue;
        }
        action_res.input_actions.push(interactive.kind.clone());
    }
}

pub fn check_damage(
    player_query: Query<(Entity, &Player)>,
    piece_query: Query<(&Parent, &Damage), With<Piece>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (parent, damage) in piece_query.iter() {
        let (player_entity, player) = player_query.get_single().unwrap();
        if !is_player_tile(&player, parent, &tile_query) {
            continue;
        }
        ev_action.send(ActionEvent(
            ActionKind::Damage(player_entity, damage.kind, damage.value)
        ));
    }
}

fn is_player_tile(
    player: &Player,
    parent: &Parent,
    tile_query: &Query<&Tile>,
) -> bool {
    let tile = tile_query.get(parent.get()).unwrap();
    tile.v == player.v
}