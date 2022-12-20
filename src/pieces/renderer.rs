use bevy::prelude::*;

use crate::globals::{PIECE_Z, TILE_SIZE};

pub fn get_piece_renderer(
    texture: &Handle<TextureAtlas>,
    idx: usize
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(idx);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = Color::WHITE;

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: texture.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., PIECE_Z)),
        ..Default::default()
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let fixture_image = asset_server.load("tiles.png");
    asset_list.0.push(fixture_image.clone_untyped());
    let fixture_atlas = TextureAtlas::from_grid(
        fixture_image,
        Vec2::splat(32.),
        1,
        4,
        None,
        None
    );
    let fixture_handle = texture_atlasses.add(fixture_atlas);

    let unit_image = asset_server.load("units.png");
    asset_list.0.push(unit_image.clone_untyped());
    let unit_atlas = TextureAtlas::from_grid(
        unit_image,
        Vec2::splat(32.),
        1,
        4,
        None,
        None
    );
    let unit_handle = texture_atlasses.add(unit_atlas);

    commands.insert_resource(PieceAssets{ 
        fixture_texture: fixture_handle,
        unit_texture: unit_handle 
    });
}

#[derive(Resource)]
pub struct PieceAssets {
    pub unit_texture: Handle<TextureAtlas>,
    pub fixture_texture: Handle<TextureAtlas>
}