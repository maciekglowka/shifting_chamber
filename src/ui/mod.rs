use bevy::prelude::*;

use crate::graphics::PieceRenderer;
use crate::states::GameState;

mod bubble;
mod game_over;
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
            .add_system(overlays::update_overlays.in_set(OnUpdate(GameState::PlayerInput)))
            .add_system(upgrade_menu::show_menu.in_schedule(OnEnter(GameState::Upgrade)))
            .add_system(upgrade_menu::clear_menu.in_schedule(OnExit(GameState::Upgrade)))
            .add_system(upgrade_menu::menu_click.in_set(OnUpdate(GameState::Upgrade)))
            .add_system(game_over::show_menu.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(game_over::clear_menu.in_schedule(OnExit(GameState::GameOver)));
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
    let font_handle = asset_server.load("pixel.ttf");
    asset_list.0.push(font_handle.clone_untyped());

    let overlay_img = asset_server.load("icons.png");
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

    let button_handle = asset_server.load("button.png");
    asset_list.0.push(button_handle.clone_untyped());

    commands.insert_resource(
        UiAssets { 
            font: font_handle,
            pico_font: pico_handle,
            overlay_texture: overlay_handle,
            button_texture: button_handle
        }
    );
}

#[derive(Default, Resource)]
pub struct UiAssets {
    font: Handle<Font>,
    pico_font: Handle<TextureAtlas>,
    overlay_texture: Handle<TextureAtlas>,
    button_texture: Handle<Image>
}

