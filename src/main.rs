#![windows_subsystem = "windows"]

use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
                    primary_window: Some(Window {
                        resolution: (
                            globals::WINDOW_WIDTH,
                            globals::WINDOW_HEIGHT
                        ).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .insert_resource(Msaa::Off)
        .add_state::<states::GameState>()
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
