use bevy::prelude::*;

use crate::states::GameState;
use crate::manager::{CommandEvent, CommandType};
use crate::vectors::Vector2Int;

mod utils;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputRes>()
            .add_system_set(
                SystemSet::on_update(GameState::PlayerInput)
                    .with_system(mouse)
            );
    }
}

// fn keys(
//     keys: ResMut<Input<KeyCode>>,
//     mut ev_command: EventWriter<CommandEvent>
// ) {
//     for (key, dir) in KEY_MAPPING {
//         if keys.just_pressed(key) {
//             ev_command.send(CommandEvent(CommandType::MapShift(-1 * dir)));
//         }
//     }
// }

// const KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
//     (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
//     (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
// ];

fn mouse (
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut res: ResMut<InputRes>,
    mut ev_command: EventWriter<CommandEvent>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(world_v) = utils::mouse_to_world(&windows, &camera_query) {
            if let Some(v) = utils::world_to_tile_position(world_v) {
                match res.selected {
                    Some(s) => {
                        ev_command.send(
                            CommandEvent(CommandType::MapShift(v, s))
                        );
                        res.selected = None;
                    },
                    None => res.selected = Some(v)
                }
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct InputRes {
    pub selected: Option<Vector2Int>
}