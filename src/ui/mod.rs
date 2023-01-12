use bevy::prelude::*;

use crate::states::GameState;

mod action_menu;
mod game_over;
mod cursor;
mod overlays;
mod sidebar;
mod upgrade_menu;

pub struct ReloadUIEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_event::<ReloadUIEvent>()
            .add_system_set(SystemSet::on_enter(GameState::PlayerInput)
                .with_system(overlays::update_overlays)
                .with_system(sidebar::update_sidebar)
                .with_system(action_menu::update_menu)
                .after("action")
            )
            .add_system_set(SystemSet::on_update(GameState::PlayerInput)
                .with_system(cursor::update_cursor)
                .with_system(action_menu::menu_click)
            )
            .add_system_set(SystemSet::on_exit(GameState::PlayerInput)
                .with_system(cursor::clear_cursor)
            )
            .add_system_set(SystemSet::on_enter(GameState::Upgrade)
                .with_system(upgrade_menu::show_menu)
            )
            .add_system_set(SystemSet::on_update(GameState::Upgrade)
                .with_system(upgrade_menu::menu_click)
            )
            .add_system_set(SystemSet::on_exit(GameState::Upgrade)
                .with_system(upgrade_menu::clear_menu)
            )
            .add_system_set(SystemSet::on_enter(GameState::GameOver)
                .with_system(game_over::show_menu)
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver)
                .with_system(game_over::clear_menu)
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

