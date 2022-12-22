use bevy::prelude::*;

use crate::actions::ActionEvent;
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Interactive,
    Piece
};

pub fn check_interactions(
    player_query: Query<&Player>,
    piece_query: Query<(&Parent, &Interactive), With<Piece>>,
    tile_query: Query<&Tile>,
    mut ev_interaction: EventWriter<ActionEvent>,
) {
    for (parent, interactive) in piece_query.iter() {
        let player = player_query.get_single().unwrap();
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }
        ev_interaction.send(ActionEvent(interactive.kind));
    }
}