use bevy::prelude::*;

mod animation;
mod assets;
mod camera;
mod common;
mod fixtures;
mod globals;
mod input;
mod manager;
mod pieces;
mod player;
mod states;
mod tiles;
mod units;
mod vectors;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    window: WindowDescriptor {
                        height: 600.,
                        width: 600.,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .add_state(states::GameState::LoadAssets)
        .add_plugin(animation::AnimationPlugin)
        .add_plugin(assets::AssetPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(manager::ManagerPlugin)
        .add_plugin(pieces::PiecesPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(tiles::TilePlugin)
        .run();
}
