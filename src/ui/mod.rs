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
mod modal;
mod overlays;
mod sidebar;
mod upgrade_menu;

pub use bubble::BubbleEvent;

pub const BG_COLOR: Color = Color::rgb(0.15, 0.25, 0.35);

#[derive(Event)]
pub struct ReloadUIEvent;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_event::<ReloadUIEvent>()
            .add_event::<bubble::BubbleEvent>()
            .add_event::<modal::CloseModalEvent>()
            .add_systems(
                Update,
                (bubble::spawn_bubbles, bubble::update_bubbles, added_piece, sidebar::update_sidebar)
            )
            .add_systems(OnEnter(GameState::PlayerInput), force_reload)
            .add_systems(OnEnter(GameState::GameOver), force_reload)
            .add_systems(
                Update,
                (overlays::update_overlays, sidebar::tile_button_click, help_menu::toggle_menu, sidebar::pause_button_click)
                    .run_if(in_state(GameState::PlayerInput))
            )
            .add_systems(Update,modal::button_click)
            .add_systems(Update,modal::clear_modal.run_if(on_event::<modal::CloseModalEvent>()))
            .add_systems(OnExit(GameState::PlayerInput), help_menu::clear_menu)
            .add_systems(OnEnter(GameState::Upgrade), upgrade_menu::show_menu)
            .add_systems(OnExit(GameState::Upgrade), upgrade_menu::clear_menu)
            .add_systems(Update, upgrade_menu::menu_click.run_if(in_state(GameState::Upgrade)))
            .add_systems(OnEnter(GameState::GameOver), game_over::show_menu)
            .add_systems(OnExit(GameState::GameOver), game_over::clear_menu)
            .add_systems(Update, game_over::menu_click.run_if(in_state(GameState::GameOver)))
            .add_systems(OnEnter(GameState::GameWin), game_win::show_menu)
            .add_systems(OnExit(GameState::GameWin), game_win::clear_menu)
            .add_systems(OnEnter(GameState::MainMenu), main_menu::show_menu)
            .add_systems(OnExit(GameState::MapInit), marker::spawn_marker)
            .add_systems(Update, marker::update_marker.run_if(in_state(GameState::PlayerInput)))
            .add_systems(OnExit(GameState::PlayerInput), marker::remove_marker);
    }  
}

pub fn no_modal(
    query: Query<&modal::Modal>
) -> bool {
    query.iter().len() == 0
}

fn force_reload(
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
    let font_handle = asset_server.load("ui/04B_03.ttf");
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

    let button_skip_handle = asset_server.load("ui/button_skip.png");
    asset_list.0.push(button_skip_handle.clone_untyped());

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
            button_skip_texture: button_skip_handle,
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
    button_skip_texture: Handle<Image>,
    tile_buttons: HashMap<TransformKind, Handle<Image>>,
    wide_button: Handle<Image>,
    title_screen: Handle<Image>
}

