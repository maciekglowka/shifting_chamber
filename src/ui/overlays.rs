use bevy::prelude::*;

use crate::globals::{OVERLAY_Z, TILE_SIZE};
use crate::pieces::components::Unit;

#[derive(Component)]
pub struct Overlay;


pub fn update_overlays(
    mut commands: Commands,
    overlay_query: Query<Entity, With<Overlay>>,
    unit_query: Query<(Entity, &Unit)>,
    assets: Res<OverlayAssets>
) {
    clear_overlays(&mut commands, &overlay_query);
    for (entity, unit) in unit_query.iter() {
        let overlay = spawn_unit_overlay(&mut commands, unit, assets.as_ref());
        commands.entity(entity)
            .add_child(overlay);
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

fn spawn_unit_overlay(
    commands: &mut Commands,
    unit: &Unit,
    assets: &OverlayAssets
) -> Entity {
    let symbols = vec!(
        // (unit.attack.value, 1),
        (unit.hp, 0),
    );
    spawn_overlay(commands, symbols, assets)
}

fn spawn_overlay(
    commands: &mut Commands,
    symbols: Vec<(u32, usize)>,
    assets: &OverlayAssets
) -> Entity {
    // let size = Vec2::new(
    //     symbols.len() as f32 * TILE_SIZE / 4.,
    //     // OVERLAY_FONT_SIZE
    // );
    // let font = assets.font.clone();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0. },
                custom_size: Some(Vec2::new(TILE_SIZE / 4., TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_translation(
                Vec3::new(TILE_SIZE * 0.375, 0., OVERLAY_Z)
            ),
            ..Default::default()
        },
        Overlay
    ))
    .with_children(|parent| {
        let size = TILE_SIZE / 8.;
        for (x, (count, sprite)) in symbols.iter().enumerate() {
            for y in 0..*count {
                let offset = Vec2::new(
                    x as f32 * size - size / 2.,
                    -(TILE_SIZE - size)/2. + y as f32 * size
                );
                parent.spawn(get_icon_bundle(
                    *sprite,
                    &assets.texture,
                    Vec2::splat(size),
                    offset
                ));
            }
        }
    })
    .id()
}

fn get_icon_bundle(
    sprite_idx: usize,
    atlas: &Handle<TextureAtlas>,
    size: Vec2,
    offset: Vec2,
) -> SpriteSheetBundle {
    let mut sprite = TextureAtlasSprite::new(sprite_idx);
    sprite.custom_size = Some(size);

    SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: atlas.clone(),
        transform: Transform::from_translation(
            Vec3::new(offset.x, offset.y, OVERLAY_Z + 1.)
        ),
        ..Default::default()
    }
}

// fn get_text_bundle(
//     text: impl Into<String>,
//     color: Color,
//     offset: Vec2,
//     font: &Handle<Font>
// ) -> Text2dBundle {
//     Text2dBundle {
//         text: Text::from_section(
//             text,
//             TextStyle {
//                 color,
//                 font: font.clone(),
//                 font_size: OVERLAY_FONT_SIZE,
//                 ..Default::default()
//             }
//         ).with_alignment(TextAlignment::CENTER_LEFT),
//         transform: Transform::from_translation(
//             Vec3::new(offset.x - TILE_SIZE / 2. + 1., offset.y, OVERLAY_Z + 1.)
//         ),
//         ..Default::default()
//     }
// }

#[derive(Resource, Default)]
pub struct OverlayAssets {
    texture: Handle<TextureAtlas>
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>    
) {
    let image = asset_server.load("icons.png");
    asset_list.0.push(image.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(8.),
        2,
        2,
        None,
        None
    );

    let atlas_handle = texture_atlasses.add(atlas);
    commands.insert_resource(OverlayAssets{ texture: atlas_handle });
}