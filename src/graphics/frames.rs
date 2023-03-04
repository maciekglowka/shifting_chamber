use bevy::prelude::*;

use crate::data::{SpriteData, SpriteColumns};

#[derive(Resource)]
pub struct SpriteTimer(Timer);
impl SpriteTimer {
    pub fn new() -> SpriteTimer {
        SpriteTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct Frames {
    base_idx: usize,
    current_idx: usize,
    frame_count: usize
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

pub fn animate_frames(
    mut query: Query<(&mut Frames, &mut TextureAtlasSprite)>,
    mut timer: ResMut<SpriteTimer>,
    time: Res<Time>
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for (mut frames, mut sprite) in query.iter_mut() {
            frames.current_idx = (frames.current_idx + 1) % frames.frame_count;
            sprite.index = frames.current_idx + frames.base_idx;
        }
    }
}