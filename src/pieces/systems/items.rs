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

pub fn update_temp_items(
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