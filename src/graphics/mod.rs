use bevy::prelude::*;
use rand::prelude::*;

use crate::data::{SpriteData, SpriteColumns};
use crate::globals::{MAP_SIZE, PROJECTILE_Z, Y_PERSPECTIVE, TILE_SIZE, TILE_Z};
use crate::states::GameState;
use crate::vectors::Vector2Int;

mod animate;
mod assets;
mod components;
mod frames;
mod spawn;

pub use components::PieceRenderer;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum AnimationSet {
    Spawn,
    Update,
    Last
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .init_resource::<animate::AnimationRes>()
            .insert_resource(frames::SpriteTimer::new())
            .configure_set(
                AnimationSet::Update.after(AnimationSet::Spawn)
            )
            .configure_set(
                AnimationSet::Last.after(AnimationSet::Update)
            )
            .add_systems(
                (
                    spawn::spawn_piece_renderer,
                    spawn::spawn_tile_renderer,
                    spawn::spawn_projectile_renderer
                ).in_set(AnimationSet::Spawn)
            )
            .add_system(frames::animate_frames)
            .add_system(animate::update_state
                .in_set(OnUpdate(GameState::TurnStart))
            )
            .add_system(animate::update_state
                .in_set(OnUpdate(GameState::TileShift))
                .in_set(AnimationSet::Last)
            )
            .add_system(animate::update_state
                .in_set(OnUpdate(GameState::NPCAction))
                .in_set(AnimationSet::Last)
            )
            .add_system(animate::update_state
                .in_set(OnUpdate(GameState::NPCResult))
                .in_set(AnimationSet::Last)
            )
            .add_systems(
                (
                    spawn::despawn_piece_renderer,
                    spawn::despawn_tile_renderer,
                ).in_base_set(CoreSet::PostUpdate)
            )
            .add_systems(
                (animate::update_tiles, animate::update_pieces)
                .in_set(OnUpdate(GameState::TileShift))
                .in_set(AnimationSet::Update)
            )
            .add_systems(
                (animate::update_pieces, animate::update_projectiles)
                .in_set(OnUpdate(GameState::NPCAction))
                .in_set(AnimationSet::Update)
            )
            .add_system(
                animate::update_pieces.in_set(OnUpdate(GameState::NPCResult))
                .in_set(AnimationSet::Update)
            );
    }
}

const TILE_VARIANTS: usize = 4;
const SPRITE_SIZE: f32 = 32.;
const PIECE_SPRITE_COLUMNS: usize = 4;

pub fn get_world_position(v: Vector2Int, z: f32) -> Vec3 {
    let offset = if z == TILE_Z { 0. } else {TILE_SIZE * 0.25};
    Vec3::new(
        v.x as f32 * TILE_SIZE,
        v.y as f32 * TILE_SIZE * Y_PERSPECTIVE + offset,
        z + (MAP_SIZE - v.y) as f32)
}

fn get_base_piece_sprite_idx(data: &SpriteData) -> usize {
    let base = data.index * PIECE_SPRITE_COLUMNS;
    match data.columns {
        Some(SpriteColumns::Variants(i)) => {
            let mut rng = thread_rng();
            base + rng.gen_range(0..i)
        }
        _ => base
    }
}

fn get_projectile_base_position(v: Vector2Int) -> Vec3 {
    get_world_position(v, PROJECTILE_Z) + Vec3::new(0., TILE_SIZE * 0.25, 0.)
}