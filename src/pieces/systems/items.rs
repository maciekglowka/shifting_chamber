use bevy::prelude::*;

use crate::globals::MAX_ITEMS;
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
    player_query: Query<(&Player, Option<&Children>)>,
    inventory_query: Query<&Item>,
    collectable_query: Query<(Entity, &Parent), (With<Piece>, With<Collectable>)>,
    tile_query: Query<&Tile>,
    mut game_res: ResMut<GameRes>
) {
    let (_, children) = match player_query.get_single() {
        Ok(p) => p,
        _ => return
    };
    let inventory_count = match children {
        Some(children) => children.iter()
            .filter(|a| inventory_query.get(**a).is_ok())
            .count(),
        None => 0
    };
    if inventory_count >= MAX_ITEMS { return; }

    for (entity, parent) in collectable_query.iter() {    
        // query has already been checked above => safe to unwrap    
        let (player, _) = player_query.get_single().unwrap();
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }

        game_res.input_commands.push(
            CommandType::PickItem(entity)
        );
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