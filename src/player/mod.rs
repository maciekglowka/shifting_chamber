use bevy::prelude::*;

use crate::data::DataAssets;
use crate::globals::{MAP_SIZE, PIECE_Z};
use crate::states::GameState;
use crate::pieces::spawn_piece_at_v;
use crate::tiles;
use crate::vectors::Vector2Int;

// mod renderer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_exit(GameState::MapInit)
                    .with_system(spawn_player)
            );
            // .add_system_set(
            //     SystemSet::on_enter(GameState::TileShift)
            //         .with_system(unpin_player)
            // )
            // .add_system_set(
            //     SystemSet::on_exit(GameState::TileShift)
            //         .with_system(pin_player)
            // );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    tile_res: Res<tiles::TileRes>,
    data_assets: Res<DataAssets>
) {
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
    // let translation = global_transform.translation();
    commands.entity(entity).remove_parent();
    // transform.translation = translation;
}

fn pin_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    tile_res: Res<tiles::TileRes>
) {
    let v = get_player_v();
    let Some(tile_entity) = tile_res.tiles.get(&v) else { return };
    let Ok(entity) = player_query.get_single() else { return };
    // transform.translation = Vec3::new(0., 0., PIECE_Z);
    commands.entity(*tile_entity).push_children(&[entity]);
}

fn get_player_v() -> Vector2Int {
    Vector2Int::new(MAP_SIZE / 2, MAP_SIZE / 2)
}