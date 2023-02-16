use bevy::prelude::*;

use crate::globals::{MAP_SIZE, PIECE_Z};
use crate::states::GameState;
use crate::pieces::{
    components::{Damage, Health},
};
use crate::tiles;
use crate::vectors::Vector2Int;

// mod renderer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_exit(GameState::MapInit)
                    .with_system(spawn_player)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::TileShift)
                    .with_system(unpin_player)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::TileShift)
                    .with_system(pin_player)
            );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    // assets: Res<renderer::PieceAssets>,
    tile_res: Res<tiles::TileRes>

) {
    let v = get_player_v();
    let Some(tile_entity) = tile_res.tiles.get(&v) else { return };
    let entity = commands.spawn((
            Player,
            Health { max: 3, value: 3},
            // renderer::get_piece_renderer(
            //     &("units".to_string(), 0),
            //     assets.as_ref()
            // )
        ))
        .id();
    commands.entity(*tile_entity)
        .push_children(&[entity]);
}

fn unpin_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &GlobalTransform), With<Player>>
) {
    let Ok((entity, mut transform, global_transform)) = player_query.get_single_mut()
        else { return };
    let translation = global_transform.translation();
    commands.entity(entity).remove_parent();
    transform.translation = translation;
}

fn pin_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    tile_res: Res<tiles::TileRes>
) {
    let v = get_player_v();
    let Some(tile_entity) = tile_res.tiles.get(&v) else { return };
    let Ok((entity, mut transform)) = player_query.get_single_mut() else { return };
    transform.translation = Vec3::new(0., 0., PIECE_Z);
    commands.entity(*tile_entity).push_children(&[entity]);
}

fn get_player_v() -> Vector2Int {
    Vector2Int::new(MAP_SIZE / 2, MAP_SIZE / 2)
}