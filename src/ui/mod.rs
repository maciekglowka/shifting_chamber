use bevy::prelude::*;

use crate::states::GameState;

mod cursor;
mod overlays;
mod sidebar;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_system_set(SystemSet::on_enter(GameState::PlayerInput)
                .with_system(overlays::update_overlays)
                .with_system(sidebar::update_sidebar)
            )
            .add_system_set(SystemSet::on_update(GameState::PlayerInput)
                .with_system(cursor::update_cursor)
            );  
    }  
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>> 
) {
    let font_handle = asset_server.load("pixel.ttf");
    asset_list.0.push(font_handle.clone_untyped());

    let overlay_img = asset_server.load("icons.png");
    asset_list.0.push(overlay_img.clone_untyped());
    let overlay_atlas = TextureAtlas::from_grid(
        overlay_img,
        Vec2::splat(8.),
        2,
        2,
        None,
        None
    );

    let overlay_handle = texture_atlasses.add(overlay_atlas);

    let cursor_img = asset_server.load("cursor.png");
    asset_list.0.push(cursor_img.clone_untyped());
    let cursor_atlas = TextureAtlas::from_grid(
        cursor_img,
        Vec2::splat(16.),
        1,
        1,
        None,
        None
    );

    let cursor_handle = texture_atlasses.add(cursor_atlas);

    commands.insert_resource(
        UiAssets { 
            font: font_handle,
            overlay_texture: overlay_handle,
            cursor_texture: cursor_handle
        }
    );
}

#[derive(Default, Resource)]
pub struct UiAssets {
    font: Handle<Font>,
    overlay_texture: Handle<TextureAtlas>,
    cursor_texture: Handle<TextureAtlas>,
}

