use bevy::prelude::*;

use crate::data::DataAssets;
use crate::globals::{PIECE_Z, TILE_SIZE};
use crate::pieces::components::Piece;
use crate::tiles::Tile;

use super::{
    assets::GraphicsAssets,
    components::PieceRenderer
};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    piece_query: Query<(Entity, &Piece, &Parent), Added<Piece>>,
    tile_query: Query<&Tile>,
    assets: Res<GraphicsAssets>,
    data_assets: Res<DataAssets>
) {
    for (entity, piece, parent) in piece_query.iter() {
        let Ok(tile) = tile_query.get(parent.get()) else { continue };
        let data = &data_assets.pieces[&piece.name];
        let texture = &assets.piece_textures[&data.sprite.0];
        let mut sprite = TextureAtlasSprite::new(data.sprite.1);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = Vec3::new(
            tile.v.x as f32 * TILE_SIZE,
            tile.v.y as f32 * TILE_SIZE,
            PIECE_Z
        );    
        commands.spawn((
            PieceRenderer { target: entity },
            SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: texture.clone(),
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
    }
}