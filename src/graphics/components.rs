use bevy::prelude::*;

use crate::data::{SpriteData, SpriteColumns};

#[derive(Component, Debug)]
pub struct Frames {
    pub base_idx: usize,
    pub current_idx: usize,
    pub frame_count: usize
}
impl Frames {
    pub fn new(data: &SpriteData) -> Frames {
        let frame_count = match data.columns {
            Some(SpriteColumns::Frames(a)) => a,
            _ => 1
        };
        let base_idx = super::get_base_piece_sprite_idx(&data);
        Frames { current_idx: 0, frame_count, base_idx}
    }
}

#[derive(Component)]
pub struct FXRenderer {
    pub looping: bool
}

#[derive(Component)]
pub struct PieceRenderer {
    pub target: Entity
}

#[derive(Component)]
pub struct ProjectileRenderer {
    pub target: Entity,
    pub linear_position: Vec3
}

#[derive(Component)]
pub struct TileRenderer {
    pub target: Entity
}