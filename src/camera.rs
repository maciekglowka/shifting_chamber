use bevy::{
    prelude::*,
    window::WindowResized
};

use crate::globals::{
    MAP_SIZE,
    SIDEBAR_WIDTH,
    Y_PERSPECTIVE,
    TILE_SIZE,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(on_resize);
    }
}

fn setup(
    mut commands: Commands,
) {
    let mut camera = Camera2dBundle::default();
    // camera.transform.translation = Vec3::new(
    //     0.5 * (TILE_SIZE * (MAP_SIZE - 1) as f32 + SIDEBAR_WIDTH),
    //     Y_PERSPECTIVE * 0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32,
    //     camera.transform.translation.z
    // );
    commands.spawn((camera, VisibilityBundle::default()));
}

fn on_resize(
    windows: Query<&Window>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    mut ev_resize: EventReader<WindowResized>
) {
    for _ in ev_resize.iter() {
        let Ok(window) = windows.get_single() else { return };

        let scale_h = match window.resolution.width() {
            a if a >= WINDOW_WIDTH => 1.0,
            a => (WINDOW_WIDTH - SIDEBAR_WIDTH) / (a - SIDEBAR_WIDTH)
        };
        let scale_v = match window.resolution.height() {
            a if a >= WINDOW_HEIGHT => 1.0,
            a => WINDOW_HEIGHT / a
        };
        let Ok((mut projection, mut transform)) = camera_query.get_single_mut() else { return };
        let scale = scale_h.max(scale_v);
        projection.scale = scale;

        let center = Vec3::new(
            0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32 + scale * 0.5 * SIDEBAR_WIDTH,
            Y_PERSPECTIVE * 0.5 * TILE_SIZE * (MAP_SIZE - 1) as f32,
            transform.translation.z
        );
        transform.translation = center;
    }
}

// fn spawn_background(
//     parent: &mut ChildBuilder,
//     z: f32
// ) {
//     parent.spawn(
//         SpriteBundle {
//             sprite: Sprite {
//                 color: crate::ui::BG_COLOR,
//                 ..Default::default()
//             },
//             transform: Transform::from_scale(Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT, 1.))
//                 .with_translation(Vec3::new(0., 0., z)),
//             ..Default::default()
//         }
//     );
// }