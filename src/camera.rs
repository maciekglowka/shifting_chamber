use bevy::prelude::*;

use crate::globals::{MAP_SIZE, SIDEBAR_WIDTH, TILE_SIZE};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32 + SIDEBAR_WIDTH / 2.,
        0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32,
        camera.transform.translation.z
    );
    commands.spawn(camera);
}