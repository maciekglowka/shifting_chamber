use bevy::prelude::*;

use crate::globals::{OVERLAY_Z, TILE_SIZE};
use crate::graphics::get_world_position;
use crate::player::get_player_v;

const MAX_OFFSET: f32 = 0.7 * TILE_SIZE;
const MIN_OFFSET: f32 = 0.65 * TILE_SIZE;
const SPEED: f32 = 8.;

#[derive(Component, Default)]
pub struct Marker {
    pub offset: f32
}

pub fn spawn_marker(
    mut commands: Commands,
    assets: Res<super::UiAssets>
) {
    commands.spawn((
        Marker::default(),
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 2,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            texture_atlas: assets.overlay_texture.clone(),
            transform: Transform::from_translation(get_v(MAX_OFFSET)),
            ..Default::default()
        }
    ));
}

pub fn remove_marker(
    mut commands: Commands,
    query: Query<Entity, With<Marker>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_marker(
    mut query: Query<(&mut Transform, &mut Marker)>,
    time: Res<Time>,
) {
    for (mut transform, mut marker) in query.iter_mut() {
        marker.offset -= SPEED * time.delta_seconds();
        if marker.offset < MIN_OFFSET {
            marker.offset = MAX_OFFSET
        }
        transform.translation = get_v(marker.offset);
    }
}

fn get_v(offset: f32) -> Vec3 {
    let player_v = get_player_v();
    get_world_position(player_v, OVERLAY_Z) + Vec3::new(0., offset, 0.)
}