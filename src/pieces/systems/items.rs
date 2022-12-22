use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Item,
    Piece
};

pub fn pick_items(
    mut commands: Commands,
    player_query: Query<&Player>,
    item_query: Query<(Entity, &Parent, &Item), With<Piece>>,
    tile_query: Query<&Tile>
) {
    for (entity, parent, item) in item_query.iter() {        
        let player = player_query.get_single().unwrap();
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }

        commands.entity(entity).despawn_recursive();
    }
}