use bevy::prelude::*;

use crate::globals::{MAX_ANIMATION_DIST, TILE_SIZE, TILE_Z};
use crate::manager::{CommandEvent, CommandType};
use crate::states::GameState;
use crate::tiles::Tile;

const MOVEMENT_SPEED: f32 = 20.;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::on_update(GameState::TileShift)
                    .with_system(update_tiles)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Action)
                    .with_system(update_actions)
            );
    }
}

fn update_tiles(
    time: Res<Time>,
    mut query: Query<(&Tile, &mut Transform)>,
    mut ev_command: EventWriter<CommandEvent>
) {
    let mut animating = false;
    for (tile, mut transform) in query.iter_mut() {
        let target = Vec3::new(
            tile.v.x as f32 * TILE_SIZE,
            tile.v.y as f32 * TILE_SIZE,
            TILE_Z
        );
        let d = (target - transform.translation).length();
        if d > MAX_ANIMATION_DIST {
            transform.translation = transform.translation.lerp(
                target,
                MOVEMENT_SPEED * time.delta_seconds()
            );
            animating = true;
        }
    }

    if !animating {
        ev_command.send(CommandEvent(CommandType::AnimationEnd));
    }
}

fn update_actions(
    mut ev_command: EventWriter<CommandEvent>
) {
    ev_command.send(CommandEvent(CommandType::AnimationEnd));
}