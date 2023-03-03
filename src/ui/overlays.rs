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
    input_res: Res<InputRes>,
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

        if let Ok(walking) = walking_query.get(renderer.target) {
            if let Some(planned_move) = walking.planned_move {
                let marker = commands.spawn((
                        Overlay,
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(TILE_SIZE / 8., TILE_SIZE / 8.)),
                                color: Color::GOLD,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                Vec3::new(0.5 *TILE_SIZE * planned_move.x as f32, 0.5 * TILE_SIZE * planned_move.y as f32, OVERLAY_Z)
                            ),
                            ..Default::default()
                        }
                    )).id();
                commands.entity(entity).add_child(marker);

                if let Some(order) = piece_res.walking_queue.iter().position(|a| *a == renderer.target) {
                    symbols.push((order as u32 + 1, Color::GOLD));
                }
            }
        }
        if input_res.extra_info {
            let overlay = spawn_overlay(&mut commands, symbols, assets.as_ref());
            commands.entity(entity).add_child(overlay);
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

// fn spawn_unit_overlay(
//     commands: &mut Commands,
//     damage: Option<&Damage>,
//     poisonous: Option<&Poisonous>,
//     unit: &Unit,
//     assets: &super::UiAssets
// ) -> Entity {
//     let mut symbols = vec!(
//         (unit.hp(), Color::GOLD),
//     );
//     if let Some(d) = damage {
//         symbols.push((d.value, Color::WHITE))
//     }
//     if let Some(p) = poisonous {
//         symbols.push((p.value, Color::LIME_GREEN))
//     }
//     spawn_overlay(commands, symbols, assets)
// }

fn spawn_overlay(
    commands: &mut Commands,
    symbols: Vec<(u32, Color)>,
    assets: &super::UiAssets
) -> Entity {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. },
                custom_size: Some(Vec2::new(TILE_SIZE / 4., TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(
                // Vec3::new(TILE_SIZE * 0.375, 0., OVERLAY_Z)
                Vec3::new(0., 0., OVERLAY_Z)
            ),
            ..Default::default()
        },
        Overlay
    ))
    .with_children(|parent| {
        let size = TILE_SIZE / 3.;
        for (x, (count, color)) in symbols.iter().enumerate() {
            // let offset = Vec2::new(
            //     size * 0.4,
            //     - TILE_SIZE * 0.4 + y as f32 * 0.75 * size
            // );
            let offset = Vec2::new(
                0.5 * TILE_SIZE - x as f32 * 0.8 * TILE_SIZE,
                - TILE_SIZE * 0.4
            );
            let sprite_idx = 48 + count;
            parent.spawn(get_icon_bundle(
                sprite_idx as usize,
                *color,
                &assets.pico_font,
                Vec2::splat(size),
                offset
            ));
        }
    })
    .id()
}

fn get_icon_bundle(
    sprite_idx: usize,
    color: Color,
    atlas: &Handle<TextureAtlas>,
    size: Vec2,
    offset: Vec2,
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(sprite_idx);
    sprite.custom_size = Some(size);
    sprite.color = color;

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: atlas.clone(),
        transform: Transform::from_translation(
            Vec3::new(offset.x, offset.y, OVERLAY_Z + 1.)
        ),
        ..Default::default()
    }
}
