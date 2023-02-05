use bevy::prelude::*;

use crate::states::GameState;

mod bubble;
mod command_menu;
mod game_over;
mod cursor;
mod overlays;
mod sidebar;
mod upgrade_menu;

pub use bubble::BubbleEvent;

pub struct ReloadUIEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_event::<ReloadUIEvent>()
            .add_event::<bubble::BubbleEvent>()
            .add_system(bubble::spawn_bubbles)
            .add_system(bubble::update_bubbles)
            .add_system_set(SystemSet::new()
                .with_system(command_menu::update_menu)
                .with_system(overlays::update_overlays)
                .with_system(sidebar::update_sidebar)
                .before("action")
            )
            .add_system_set(SystemSet::on_enter(GameState::PlayerInput)
                .with_system(player_input)
                .after("action")
            )
            .add_system_set(SystemSet::on_update(GameState::PlayerInput)
                .with_system(cursor::update_cursor)
                .with_system(command_menu::menu_click)
                .with_system(sidebar::button_click)
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

fn player_input(
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    ev_ui.send(ReloadUIEvent);
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

    let pico_img = asset_server.load("pico.png");
    asset_list.0.push(pico_img.clone_untyped());
    let pico_atlas = TextureAtlas::from_grid(
        pico_img,
        Vec2::splat(32.),
        16,
        16,
        None,
        None
    );
    let pico_handle = texture_atlasses.add(pico_atlas);

    commands.insert_resource(
        UiAssets { 
            font: font_handle,
            pico_font: pico_handle,
            overlay_texture: overlay_handle,
            cursor_texture: cursor_handle
        }
    );
}

#[derive(Default, Resource)]
pub struct UiAssets {
    font: Handle<Font>,
    pico_font: Handle<TextureAtlas>,
    overlay_texture: Handle<TextureAtlas>,
    cursor_texture: Handle<TextureAtlas>,
}

