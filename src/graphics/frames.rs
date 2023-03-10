use bevy::prelude::*;

use super::components::{FXRenderer, Frames};

const FRAME_DURATION: f32 = 0.5;
const FX_FRAME_DURATION: f32 = 0.1;

#[derive(Resource)]
pub struct SpriteTimer(Timer);
impl SpriteTimer {
    pub fn new() -> SpriteTimer {
        SpriteTimer(Timer::from_seconds(FRAME_DURATION, TimerMode::Repeating))
    }
}

pub fn animate_frames(
    mut query: Query<(&mut Frames, &mut TextureAtlasSprite), Without<FXRenderer>>,
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

#[derive(Resource)]
pub struct FXTimer(Timer);
impl FXTimer {
    pub fn new() -> FXTimer {
        FXTimer(Timer::from_seconds(FX_FRAME_DURATION, TimerMode::Repeating))
    }
}

pub fn animate_fx_frames(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Frames, &mut TextureAtlasSprite, &FXRenderer)>,
    mut timer: ResMut<FXTimer>,
    time: Res<Time>
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for (entity, mut frames, mut sprite, renderer) in query.iter_mut() {
            if !renderer.looping && frames.current_idx == frames.frame_count - 1 {
                commands.entity(entity).despawn_recursive();
                continue;
            }
            frames.current_idx = (frames.current_idx + 1) % frames.frame_count;
            sprite.index = frames.current_idx + frames.base_idx;
        }
    }
}