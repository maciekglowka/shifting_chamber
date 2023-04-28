use bevy::prelude::*;
use std::collections::HashMap;

use crate::graphics::PieceRenderer;
use crate::player::upgrades::TransformKind;
use crate::states::GameState;

mod bubble;
mod game_over;
mod game_win;
mod help_menu;
mod main_menu;
mod marker;
mod overlays;
mod sidebar;
mod upgrade_menu;

pub use bubble::BubbleEvent;

pub const BG_COLOR: Color = Color::rgb(0.15, 0.25, 0.35);
pub struct ReloadUIEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_event::<ReloadUIEvent>()
            .add_event::<bubble::BubbleEvent>()
            .add_system(bubble::spawn_bubbles)
            .add_system(bubble::update_bubbles)
            .add_system(added_piece)
            .add_system(sidebar::update_sidebar)
            .add_system(player_input.in_schedule(OnEnter(GameState::PlayerInput)))
            .add_systems(
                (overlays::update_overlays, sidebar::tile_button_click, help_menu::toggle_menu, sidebar::pause_button_click)
                .in_set(OnUpdate(GameState::PlayerInput))
            )
            .add_system(help_menu::clear_menu.in_schedule(OnExit(GameState::PlayerInput)))
            .add_system(upgrade_menu::show_menu.in_schedule(OnEnter(GameState::Upgrade)))
            .add_system(upgrade_menu::clear_menu.in_schedule(OnExit(GameState::Upgrade)))
            .add_system(upgrade_menu::menu_click.in_set(OnUpdate(GameState::Upgrade)))
            .add_system(game_over::show_menu.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(game_over::clear_menu.in_schedule(OnExit(GameState::GameOver)))
            .add_system(game_over::menu_click.in_set(OnUpdate(GameState::GameOver)))
            .add_system(game_win::show_menu.in_schedule(OnEnter(GameState::GameWin)))
            .add_system(game_win::clear_menu.in_schedule(OnExit(GameState::GameWin)))
            .add_system(main_menu::show_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(main_menu::clear_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(marker::spawn_marker.in_schedule(OnExit(GameState::MapInit)))
            .add_system(marker::update_marker.in_set(OnUpdate(GameState::PlayerInput)))
            .add_system(marker::remove_marker.in_schedule(OnExit(GameState::PlayerInput)));
    }  
}

fn player_input(
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    ev_ui.send(ReloadUIEvent);
}

fn added_piece(
    mut ev_ui: EventWriter<ReloadUIEvent>,
    query: Query<Entity, Added<PieceRenderer>>
) {
    for _ in query.iter() {
        ev_ui.send(ReloadUIEvent);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>> 
) {
    let font_handle = asset_server.load("ui/PICO-8 mono.ttf");
    asset_list.0.push(font_handle.clone_untyped());

    let overlay_img = asset_server.load("ui/icons.png");
    asset_list.0.push(overlay_img.clone_untyped());
    let overlay_atlas = TextureAtlas::from_grid(
        overlay_img,
        Vec2::splat(32.),
        1,
        4,
        None,
        None
    );

    let overlay_handle = texture_atlasses.add(overlay_atlas);

    let pico_img = asset_server.load("ui/pico.png");
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

    let button_handle = asset_server.load("ui/button.png");
    asset_list.0.push(button_handle.clone_untyped());

    let shift = asset_server.load("ui/shift.png");
    asset_list.0.push(shift.clone_untyped());
    let switch = asset_server.load("ui/switch.png");
    asset_list.0.push(switch.clone_untyped());
    let rotate = asset_server.load("ui/rotate.png");
    asset_list.0.push(rotate.clone_untyped());
    let tiles = HashMap::from([
        (TransformKind::TileShift, shift),
        (TransformKind::TileSwitch, switch),
        (TransformKind::TileRotate, rotate),
    ]);

    let title = asset_server.load("ui/title.png");
    asset_list.0.push(title.clone_untyped());

    let wide_handle = asset_server.load("ui/wide_button.png");
    asset_list.0.push(wide_handle.clone_untyped());

    commands.insert_resource(
        UiAssets { 
            font: font_handle,
            pico_font: pico_handle,
            overlay_texture: overlay_handle,
            button_texture: button_handle,
            tile_buttons: tiles,
            title_screen: title,
            wide_button: wide_handle
        }
    );
}

#[derive(Default, Resource)]
pub struct UiAssets {
    font: Handle<Font>,
    pico_font: Handle<TextureAtlas>,
    overlay_texture: Handle<TextureAtlas>,
    button_texture: Handle<Image>,
    tile_buttons: HashMap<TransformKind, Handle<Image>>,
    wide_button: Handle<Image>,
    title_screen: Handle<Image>
}

