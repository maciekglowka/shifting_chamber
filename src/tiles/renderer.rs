use bevy::prelude::*;

use crate::globals::{TILE_SIZE, TILE_Z};
use crate::vectors::Vector2Int;

pub fn get_tile_renderer(
    v: Vector2Int,
    assets: &TileAssets
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: assets.texture.clone(),
        transform: Transform::from_translation(
            Vec3::new(v.x as f32 * TILE_SIZE, v.y as f32 * TILE_SIZE, TILE_Z)
        ),
        ..Default::default()
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let image = asset_server.load("tiles.png");
    asset_list.0.push(image.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(32.),
        1,
        4,
        None,
        None
    );

    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(TileAssets{ texture: atlas_handle });
}

#[derive(Resource)]
pub struct TileAssets {
    texture: Handle<TextureAtlas>
}