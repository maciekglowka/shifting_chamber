use bevy::prelude::*;
use std::collections::HashMap;

const PIECE_SPRITE_FILES: [(&str, usize, usize); 4] = [
    // atlas, columns, rows
    ("fixtures", 1, 4),
    ("items", 1, 4),
    ("tiles", 1, 4),
    ("units", 1, 4)
];

#[derive(Resource)]
pub struct GraphicsAssets {
    pub piece_textures: HashMap<String, Handle<TextureAtlas>>
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let mut piece_textures = HashMap::new();

    for (fname, columns, rows) in PIECE_SPRITE_FILES {
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
        piece_textures.insert(fname.to_string(), handle);
    }

    commands.insert_resource(GraphicsAssets{ 
        piece_textures
    });
}