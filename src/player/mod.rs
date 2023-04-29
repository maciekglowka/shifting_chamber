use bevy::prelude::*;

use crate::data::DataAssets;
use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::pieces::{
    components::Piece,
    spawn_piece_at_v
};
use crate::tiles;
use crate::vectors::Vector2Int;

pub mod upgrades;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                (spawn_player, pin_player)
                .in_schedule(OnExit(GameState::MapInit))
            )
            .add_system(unpin_player.in_schedule(OnEnter(GameState::MapEnd)))
            .add_system(unpin_player.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(despawn_player.in_schedule(OnEnter(GameState::MainMenu)));
    }
}

#[derive(Component)]
pub struct Player;

fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_player(
    mut commands: Commands,
    player_query: Query<&Player>,
    tile_res: Res<tiles::TileRes>,
    data_assets: Res<DataAssets>
) {
    if player_query.get_single().is_ok() {
        // do not spawn the player if exists,
        return;
    }
    let entity = spawn_piece_at_v(
        &mut commands,
        "Player".to_string(),
        get_player_v(),
        tile_res.as_ref(),
        data_assets.as_ref()
    );
    commands.entity(entity).insert(Player);
}

fn unpin_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    let Ok(entity) = player_query.get_single()
        else { return };
    commands.entity(entity).remove_parent();
    commands.entity(entity).remove::<Piece>();
}

fn pin_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    tile_res: Res<tiles::TileRes>
) {
    let v = get_player_v();
    let Some(tile_entity) = tile_res.tiles.get(&v) else { return };
    let Ok(entity) = player_query.get_single() else { return };
    commands.entity(*tile_entity).push_children(&[entity]);
    commands.entity(entity).insert(Piece { name: "Player".to_string()} );
}

pub fn get_player_v() -> Vector2Int {
    Vector2Int::new(MAP_SIZE / 2, MAP_SIZE / 2)
}