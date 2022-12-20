use bevy::prelude::*;

use crate::globals::{TILE_SIZE, OVERLAY_Z};
use crate::input::InputRes;

#[derive(Component)]
pub struct Cursor;


pub fn update_cursor(
    mut commands: Commands,
    res: Res<InputRes>,
    assets: Res<CursorAssets>,
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
                texture_atlas: assets.texture.clone(),
                transform: Transform::from_translation(
                    Vec3::new(v.x as f32 * TILE_SIZE, v.y as f32 * TILE_SIZE, OVERLAY_Z)
                ),
                ..Default::default()
            }
        ));
    } else {
        clear_cursor(&mut commands, &query);
    }
}

fn clear_cursor(
    commands: &mut Commands,
    query: &Query<Entity, With<Cursor>>
) {
    for cursor in query.iter() {
        commands.entity(cursor)
            .despawn_recursive();
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let image = asset_server.load("cursor.png");
    asset_list.0.push(image.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(16.),
        1,
        1,
        None,
        None
    );

    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(CursorAssets{ texture: atlas_handle });
}

#[derive(Resource)]
pub struct CursorAssets {
    texture: Handle<TextureAtlas>
}