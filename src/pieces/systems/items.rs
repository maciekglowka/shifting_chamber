use bevy::prelude::*;

// use crate::actions::{ActionEvent, ActionKind};
use crate::manager::{CommandType, GameRes};
use crate::player::Player;
use crate::tiles::Tile;

use super::super::components::{
    Collectable,
    Item,
    Piece,
    Temporary
};

pub fn examine_pickable_items(
    player_query: Query<&Player>,
    item_query: Query<(Entity, &Parent), (With<Piece>, With<Collectable>)>,
    tile_query: Query<&Tile>,
    mut game_res: ResMut<GameRes>
) {
    for (entity, parent) in item_query.iter() {        
        let player = match player_query.get_single() {
            Ok(p) => p,
            _ => return
        };
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }

        game_res.input_actions.push(CommandType::PickItem(entity));
    }
}

pub fn remove_disposable_items(
    mut commands: Commands,
    player_query: Query<&Player>,
    item_query: Query<(Entity, &Parent), (With<Piece>, With<Item>, Without<Collectable>)>,
    tile_query: Query<&Tile>
) {
    // remove non collectable items, when player is standing on them
    for (entity, parent) in item_query.iter() {        
        let player = match player_query.get_single() {
            Ok(p) => p,
            _ => return
        };
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }

        commands.entity(entity).despawn_recursive();
    }
}
