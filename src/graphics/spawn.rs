use bevy::prelude::*;
use rand::prelude::*;

use crate::data::{DataAssets, SpriteColumns};
use crate::globals::{PIECE_Z, PROJECTILE_Z, TILE_SIZE, TILE_Z};
use crate::pieces::components::{Piece, Projectile};
use crate::tiles::Tile;

use super::{
    assets::GraphicsAssets,
    components::{PieceRenderer, ProjectileRenderer, TileRenderer}
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
        let texture = &assets.piece_textures[&data.sprite.atlas];
        let mut sprite = TextureAtlasSprite::new(super::get_base_piece_sprite_idx(&data.sprite));
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let v = super::get_world_position(tile.v, PIECE_Z);
        let renderer = commands.spawn((
                PieceRenderer { target: entity },
                SpriteSheetBundle {
                    sprite: sprite,
                    texture_atlas: texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            ))
            .id();
        match data.sprite.columns {
            Some(SpriteColumns::Frames(_)) => {
                commands.entity(renderer)
                    .insert(super::frames::Frames::new(&data.sprite));
                },
            _ => ()
        }
    }
}

pub fn despawn_piece_renderer(
    mut commands: Commands,
    removed: RemovedComponents<Piece>,
    renderer_query: Query<(Entity, &PieceRenderer)>
) {
    for parent_entity in removed.iter() {
        for (entity, renderer) in renderer_query.iter() {
            if parent_entity != renderer.target { continue };
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_tile_renderer(
    mut commands: Commands,
    tile_query: Query<(Entity, &Tile), Added<Tile>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, tile) in tile_query.iter() {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..super::TILE_VARIANTS);
        let texture = &assets.tile_texture;
        let mut sprite = TextureAtlasSprite::new(idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let v = super::get_world_position(tile.v, TILE_Z);
        commands.spawn((
            TileRenderer { target: entity },
            SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: texture.clone(),
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
    }
}

pub fn despawn_tile_renderer(
    mut commands: Commands,
    removed: RemovedComponents<Tile>,
    renderer_query: Query<(Entity, &TileRenderer)>
) {
    for parent_entity in removed.iter() {
        for (entity, renderer) in renderer_query.iter() {
            if parent_entity != renderer.target { continue };
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_projectile_renderer(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile), Added<Projectile>>,
    assets: Res<GraphicsAssets>,
    mut animation_res: ResMut<super::animate::AnimationRes>
) {
    for (entity, projectile) in projectile_query.iter() {
        let texture = &assets.elements_texture;
        let mut sprite = TextureAtlasSprite::new(0);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let v = super::get_world_position(projectile.source, PROJECTILE_Z);
        commands.spawn((
            ProjectileRenderer { target: entity, linear_position: v },
            SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: texture.clone(),
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
        animation_res.is_animating = true;
    }
}

