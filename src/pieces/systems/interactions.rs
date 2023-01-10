use bevy::prelude::*;

use crate::actions::ActionEvent;
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Interactive,
    Piece,
    Temporary
};

pub fn check_interactions(
    player_query: Query<&Player>,
    piece_query: Query<(&Parent, &Interactive), With<Piece>>,
    tile_query: Query<&Tile>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for (parent, interactive) in piece_query.iter() {
        let player = player_query.get_single().unwrap();
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }
        ev_action.send(ActionEvent(interactive.kind.clone()));
    }
}

pub fn update_temporary(
    mut commands: Commands,
    mut item_query: Query<(Entity, &mut Temporary), Without<Piece>>,
) {
    for (entity, mut temporary) in item_query.iter_mut() {        
        temporary.value = temporary.value.saturating_sub(1);
        if temporary.value == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
