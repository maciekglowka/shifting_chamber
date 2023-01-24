use bevy::prelude::*;
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;

use crate::globals::{PIECE_Z, TILE_SIZE};

const SPRITE_FILES: [(&str, usize, usize); 4] = [
    // atlas, columns, rows
    ("fixtures", 1, 4),
    ("items", 1, 4),
    ("tiles", 1, 4),
    ("units", 1, 4)
];

pub fn get_piece_renderer(
    sprite_data: &(String, usize),
    assets: &PieceAssets,
) -> SpriteSheetBundle {
    let texture = &assets.textures[&sprite_data.0];
    let mut sprite = TextureAtlasSprite::new(sprite_data.1);
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
    let mut textures = HashMap::new();

    for (fname, columns, rows) in SPRITE_FILES {
        let image = asset_server.load(fname.to_owned() + ".png");
        asset_list.0.push(image.clone_untyped());
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::splat(32.),
            columns,
            rows,
            None,
            None
        );
        let handle = texture_atlasses.add(atlas);
        textures.insert(fname.to_string(), handle);
    }

    commands.insert_resource(PieceAssets{ textures });
}

#[derive(Resource)]
pub struct PieceAssets {
    pub textures: HashMap<String, Handle<TextureAtlas>>
}