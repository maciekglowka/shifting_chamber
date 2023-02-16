use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::manager::{CommandType, GameRes};
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Damage
};

// pub fn check_damage(
//     player_query: Query<(Entity, &Player)>,
//     piece_query: Query<(&Parent, &Damage), With<Piece>>,
//     tile_query: Query<&Tile>,
//     mut ev_action: EventWriter<ActionEvent>,
// ) {
//     for (parent, damage) in piece_query.iter() {
//         let (player_entity, player) = player_query.get_single().unwrap();
//         if !is_player_tile(&player, parent, &tile_query) {
//             continue;
//         }
//         ev_action.send(ActionEvent(
//             ActionKind::Damage(player_entity, damage.kind, damage.value)
//         ));
//     }
// }
