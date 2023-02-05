use bevy::prelude::*;

use crate::globals::{
    MAP_SIZE,
    SIDEBAR_WIDTH,
    TILE_SIZE,
    WINDOW_HEIGHT,
    WINDOW_WIDTH
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands
) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32,
        0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32,
        camera.transform.translation.z
    );
    let bg_z = -camera.transform.translation.z;
    commands.spawn((camera, VisibilityBundle::default()))
        .with_children(|parent| spawn_background(parent, bg_z));
}

fn spawn_background(
    parent: &mut ChildBuilder,
    z: f32
) {
    parent.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                ..Default::default()
            },
            transform: Transform::from_scale(Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT, 1.))
                .with_translation(Vec3::new(0., 0., z)),
            ..Default::default()
        }
    );
}