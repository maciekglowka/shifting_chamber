#![windows_subsystem = "windows"]

use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

mod actions;
mod assets;
mod camera;
mod data;
mod globals;
mod graphics;
mod input;
mod manager;
mod pieces;
mod player;
mod states;
mod tiles;
mod ui;
mod vectors;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    app.add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    window: WindowDescriptor {
                        height: globals::WINDOW_HEIGHT,
                        width: globals::WINDOW_WIDTH,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .add_state(states::GameState::LoadAssets)
        .add_plugin(actions::ActionPlugin)
        .add_plugin(assets::AssetPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(data::DataPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(manager::ManagerPlugin)
        .add_plugin(pieces::PiecesPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(tiles::TilePlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
