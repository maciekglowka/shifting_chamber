use bevy::prelude::*;

use crate::states::GameState;
use crate::manager::{CommandEvent, CommandType};
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerInput)
                .with_system(keys)
        );
    }
}

fn keys(
    keys: ResMut<Input<KeyCode>>,
    mut ev_command: EventWriter<CommandEvent>
) {
    for (key, dir) in KEY_MAPPING {
        if keys.just_pressed(key) {
            ev_command.send(CommandEvent(CommandType::MapShift(dir)));
        }
    }
}

const KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
];