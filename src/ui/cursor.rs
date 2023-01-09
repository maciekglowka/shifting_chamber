use bevy::prelude::*;

use crate::globals::{TILE_SIZE, OVERLAY_Z};
use crate::input::InputRes;

#[derive(Component)]
pub struct Cursor;


pub fn update_cursor(
    mut commands: Commands,
    res: Res<InputRes>,
    assets: Res<super::UiAssets>,
    query: Query<Entity, With<Cursor>>
) {
    if !res.is_changed() { return; }

    if let Some(v) = res.selected {
        let mut sprite = TextureAtlasSprite::new(0);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        commands.spawn((
            Cursor,
            SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: assets.cursor_texture.clone(),
                transform: Transform::from_translation(
                    Vec3::new(v.x as f32 * TILE_SIZE, v.y as f32 * TILE_SIZE, OVERLAY_Z)
                ),
                ..Default::default()
            }
        ));
    } else {
        destroy_cursor(&mut commands, &query);
    }
}

pub fn clear_cursor(
    mut commands: Commands,
    query: Query<Entity, With<Cursor>>
) {
    destroy_cursor(&mut commands, &query);
}

fn destroy_cursor(
    commands: &mut Commands,
    query: &Query<Entity, With<Cursor>>
) {
    for cursor in query.iter() {
        commands.entity(cursor)
            .despawn_recursive();
    }
}
