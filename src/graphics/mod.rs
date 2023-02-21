use bevy::prelude::*;

use crate::states::GameState;

mod animate;
mod assets;
mod components;
mod spawn;

pub use components::PieceRenderer;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(spawn::spawn_piece_renderer)
            .add_system_to_stage(
                CoreStage::PostUpdate, spawn::despawn_piece_renderer
            )
            .add_system_set(
                SystemSet::on_update(GameState::TurnStart)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::TileShift)
                    .with_system(animate::update_tiles)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::NPCMove)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::MoveResult)
                    .with_system(animate::update_pieces)
            )
            .add_system_set(
                SystemSet::on_update(GameState::TurnEnd)
                    .with_system(animate::update_pieces)
            );
    }
}

// fn update_tiles(
//     time: Res<Time>,
//     mut query: Query<(&Tile, &mut Transform)>,
//     mut ev_command: EventWriter<CommandEvent>
// ) {
//     let mut animating = false;
//     for (tile, mut transform) in query.iter_mut() {
//         let target = Vec3::new(
//             tile.v.x as f32 * TILE_SIZE,
//             tile.v.y as f32 * TILE_SIZE,
//             TILE_Z
//         );
//         let d = (target - transform.translation).length();
//         if d > MAX_ANIMATION_DIST {
//             transform.translation = transform.translation.lerp(
//                 target,
//                 MOVEMENT_SPEED * time.delta_seconds()
//             );
//             animating = true;
//         } else {
//             transform.translation = target;
//         }
//     }

//     if !animating {
//         ev_command.send(CommandEvent(CommandType::AnimationEnd));
//     }
// }

// fn update_animated(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut ev_command: EventWriter<CommandEvent>,
//     mut animated_query: Query<(Entity, &mut Animated, &mut Transform, &GlobalTransform)>
// ) {
//     let mut animating = false;
//     for (entity, mut animated, mut transform, global_transform) in animated_query.iter_mut() {
//         let Some(v) = animated.path.get(0) else { 
//             commands.entity(entity)
//                 .remove::<Animated>();
//             continue;
//          };
//         let abs_target = Vec3::new(
//             v.x as f32 * TILE_SIZE,
//             v.y as f32 * TILE_SIZE,
//             global_transform.translation().z
//         );
//         let mut local_target = abs_target - global_transform.translation();
//         local_target.z = transform.translation.z;
//         let d = (local_target - transform.translation).length();
//         if d > MAX_ANIMATION_DIST {
//             transform.translation = transform.translation.lerp(
//                 local_target,
//                 MOVEMENT_SPEED * time.delta_seconds()
//             );
//         } else {
//             transform.translation = local_target;
//             animated.path.pop_front();
//         }
//         animating = true;
//     }

//     if !animating {
//         ev_command.send(CommandEvent(CommandType::AnimationEnd))
//     }
// }

// #[derive(Component)]
// struct Animated {
//     pub path: VecDeque<Vector2Int>
// }
