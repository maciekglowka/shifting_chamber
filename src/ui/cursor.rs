use bevy::prelude::*;

use crate::globals::{TILE_SIZE, CURSOR_Z};
use crate::input::InputRes;
use crate::pieces::components::Unit;
use crate::player::Player;
use crate::tiles::{can_shift, TileRes};
use crate::vectors::ORTHO_DIRECTIONS;

#[derive(Component)]
pub struct Cursor;


pub fn update_cursor(
    mut commands: Commands,
    res: Res<InputRes>,
    assets: Res<super::UiAssets>,
    query: Query<Entity, With<Cursor>>,
    unit_query: Query<&Parent, With<Unit>>,
    tile_res: Res<TileRes>,
    player_query: Query<&Player>
) {
    if !res.is_changed() { return; }

    if let Some(v) = res.selected {
        let mut sprite = TextureAtlasSprite::new(0);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let mut tiles = vec!((v, Color::SILVER));

        if let Ok(player) = player_query.get_single() {
            for dir in ORTHO_DIRECTIONS {
                if can_shift(
                    v,
                    dir,
                    player.v,
                    &unit_query,
                    tile_res.as_ref()
                ) { tiles.push((v + dir, Color::GRAY)) }
            }
        }

        for (t, color) in tiles {
            let mut s = sprite.clone();
            s.color = color;
            commands.spawn((
                Cursor,
                SpriteSheetBundle {
                    sprite: s,
                    texture_atlas: assets.cursor_texture.clone(),
                    transform: Transform::from_translation(
                        Vec3::new(t.x as f32 * TILE_SIZE, t.y as f32 * TILE_SIZE, CURSOR_Z)
                    ),
                    ..Default::default()
                }
            ));
        }
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
