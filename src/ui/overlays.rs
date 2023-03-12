use bevy::prelude::*;

use crate::globals::{OVERLAY_Z, TILE_SIZE};
use crate::graphics::{get_world_position, PieceRenderer};
use crate::input::InputRes;
use crate::pieces::{
    components::{Health, Walking},
    PieceRes
};

#[derive(Component)]
pub struct Overlay;

pub fn update_overlays(
    mut commands: Commands,
    overlay_query: Query<Entity, With<Overlay>>,
    renderer_query: Query<(Entity, &PieceRenderer)>,
    walking_query: Query<&Walking>,
    health_query: Query<&Health>,
    piece_res: Res<PieceRes>,
    assets: Res<super::UiAssets>,
    mut ev_ui: EventReader<super::ReloadUIEvent>
) {
    if ev_ui.iter().len() == 0 { return };
    // even if there are multiple events, run only once per frame
    clear_overlays(&mut commands, &overlay_query);
    for (entity, renderer) in renderer_query.iter() {
        let mut symbols = Vec::new();
        if let Ok(health) = health_query.get(renderer.target) {
            symbols.push((health.value, Color::MAROON));
        }
        let symbol_overlay = spawn_symbols_overlay(&mut commands, symbols, assets.as_ref());
        commands.entity(entity).add_child(symbol_overlay);

        if let Ok(walking) = walking_query.get(renderer.target) {
            spawn_walk_overlay(&mut commands, walking, assets.as_ref(), entity);
            if let Some(order) = piece_res.action_queue.iter().position(|a| *a == renderer.target) {
                spawn_order_overlay(&mut commands, entity, assets.as_ref(), order + 1);
                // symbols.push((order as u32 + 1, Color::WHITE));
            }
        }

    }
}

fn clear_overlays(
    commands: &mut Commands,
    query: &Query<Entity, With<Overlay>>
) {
    for overlay in query.iter() {
        commands.entity(overlay)
            .despawn_recursive();
    }
}

fn spawn_walk_overlay(
    commands: &mut Commands,
    walking: &Walking,
    assets: &super::UiAssets,
    parent: Entity
)
{
    if let Some(planned_move) = walking.planned_move {
        let angle = Vec2::new(1., 0.)
            .angle_between(Vec2::new(planned_move.x as f32, planned_move.y as f32));
        let rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        let marker = commands.spawn((
                Overlay,
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 0,
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..Default::default()
                    },
                    texture_atlas: assets.overlay_texture.clone(),
                    transform: Transform::from_rotation(rotation)
                        .with_translation(
                            Vec3::new(0.5 * TILE_SIZE * planned_move.x as f32, 0.5 * TILE_SIZE * planned_move.y as f32, OVERLAY_Z)
                        ),
                    ..Default::default()
                }
            )).id();
        commands.entity(parent).add_child(marker);
    }
}

fn spawn_order_overlay(
    commands: &mut Commands,
    parent: Entity,
    assets: &super::UiAssets,
    order: usize
) {
    let entity = commands.spawn((
        Overlay,
        get_icon_bundle(
            48 + order,
            Color::WHITE, 
            &assets.pico_font,
            Vec2::splat(TILE_SIZE / 4.),
            Vec3::new(TILE_SIZE * 7. / 16., - TILE_SIZE * 3. / 8., OVERLAY_Z)
        ))
    ).id();
    commands.entity(parent).add_child(entity);
}

fn spawn_symbols_overlay(
    commands: &mut Commands,
    symbols: Vec<(u32, Color)>,
    assets: &super::UiAssets
) -> Entity {
    let bar_width = TILE_SIZE / 2.;
    let bar_height = TILE_SIZE / 8.;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. },
                custom_size: Some(Vec2::new(bar_width, bar_height)),
                ..Default::default()
            },
            transform: Transform::from_translation(
                Vec3::new(0., -TILE_SIZE * 3. / 8., 0.)
            ),
            ..Default::default()
        },
        Overlay
    ))
    .with_children(|parent| {
        let size = TILE_SIZE / 8.;
        let cell_size = 0.75 * size;
        for (i, (count, color)) in symbols.iter().enumerate() {
            let base_offset = Vec3::new(
                - 0.5 * (count-1) as f32 * cell_size,
                i as f32 * 1.5 * size,
                0.
            );
            for j in 0..*count {
                let offset = base_offset + Vec3::new(j as f32 * cell_size, 0., 0.);
                parent.spawn(
                    get_icon_bundle(
                        135,
                        *color, 
                        &assets.pico_font,
                        Vec2::splat(size),
                        offset
                    )
                );
            }
        }
    })
    .id()
}

fn get_icon_bundle(
    sprite_idx: usize,
    color: Color,
    atlas: &Handle<TextureAtlas>,
    size: Vec2,
    offset: Vec3,
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(sprite_idx);
    sprite.custom_size = Some(size);
    sprite.color = color;

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: atlas.clone(),
        transform: Transform::from_translation(
            offset
            // Vec3::new(offset.x, offset.y, OVERLAY_Z + 1.)
        ),
        ..Default::default()
    }
}
