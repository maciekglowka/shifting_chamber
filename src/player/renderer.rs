use bevy::prelude::*;

use crate::globals::{MAP_SIZE, PIECE_Z, TILE_SIZE};

pub fn get_renderer(
    assets: &PlayerAssets
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = Color::WHITE;

    let d = (MAP_SIZE / 2) as f32 * TILE_SIZE;

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: assets.texture.clone(),
        transform: Transform::from_translation(
            Vec3::new(d, d, PIECE_Z)
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
    let image = asset_server.load("ascii.png");
    asset_list.0.push(image.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        None
    );

    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(PlayerAssets{ texture: atlas_handle });
}

#[derive(Resource)]
pub struct PlayerAssets {
    texture: Handle<TextureAtlas>
}