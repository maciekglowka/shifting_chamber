use bevy::prelude::*;
use std::collections::HashMap;

use super::PIECE_SPRITE_COLUMNS;
const PIECE_SPRITE_FILES: [(&str, usize, usize); 2] = [
    // atlas, columns, rows
    ("fixtures", PIECE_SPRITE_COLUMNS, 4),
    ("units", PIECE_SPRITE_COLUMNS, 4)
];

#[derive(Resource)]
pub struct GraphicsAssets {
    pub piece_textures: HashMap<String, Handle<TextureAtlas>>,
    pub tile_texture: Handle<TextureAtlas>,
    pub elements_texture: Handle<TextureAtlas>,
    pub fx_texture: Handle<TextureAtlas>
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let mut piece_textures = HashMap::new();

    for (fname, columns, rows) in PIECE_SPRITE_FILES {
        let handle = load_texture_file(
            &(fname.to_string() + ".png"),
            columns,
            rows,
            asset_server.as_ref(),
            texture_atlasses.as_mut(),
            asset_list.as_mut()
        );
        piece_textures.insert(fname.to_string(), handle);
    }

    let tile_texture = load_texture_file(
        "tiles.png",
        1,
        super::TILE_VARIANTS,
        asset_server.as_ref(),
        texture_atlasses.as_mut(),
        asset_list.as_mut()
    );

    let elements_texture = load_texture_file(
        "elements.png",
        1,
        4,
        asset_server.as_ref(),
        texture_atlasses.as_mut(),
        asset_list.as_mut()
    );

    let fx_texture = load_texture_file(
        "fx.png",
        4,
        4,
        asset_server.as_ref(),
        texture_atlasses.as_mut(),
        asset_list.as_mut()
    );

    commands.insert_resource(GraphicsAssets{ 
        piece_textures,
        tile_texture,
        elements_texture,
        fx_texture
    });
}

fn load_texture_file(
    fname: &str,
    columns: usize,
    rows: usize,
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
    asset_list: &mut crate::assets::AssetList
) -> Handle<TextureAtlas> {
    let image = asset_server.load(fname);
    asset_list.0.push(image.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(super::SPRITE_SIZE),
        columns,
        rows,
        None,
        None
    );
    texture_atlasses.add(atlas)
}