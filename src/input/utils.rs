use bevy::prelude::*;

use crate::globals::{MAP_SIZE, TILE_SIZE};
use crate::vectors::Vector2Int;

pub fn mouse_to_world(
    windows: &Query<&Window>,
    camera_query: &Query<(&Camera, &GlobalTransform)>
) -> Option<Vec2> {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(c) => c,
        Err(_) => return None
    };
    let Ok(window) = windows.get_single() else { return None };

    let screen_pos = window.cursor_position()?;
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (screen_pos / window_size) * 2. - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    Some(ndc_to_world.project_point3(ndc.extend(-1.)).truncate())
}

pub fn world_to_tile_position(v: Vec2) -> Option<Vector2Int> {
    let x = (v.x / TILE_SIZE - 0.5).ceil() as i32;
    let y = (v.y / TILE_SIZE - 0.5).ceil() as i32;

    if x>=0 && y>=0 && x < MAP_SIZE && y < MAP_SIZE {
        Some(Vector2Int::new(x, y))
    } else {
        None
    }
}

