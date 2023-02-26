use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::states::GameState;
use crate::manager::{CommandEvent, CommandType};
use crate::ui::ReloadUIEvent;
use crate::vectors::Vector2Int;
use crate::tiles::transform::TileTransform;

mod utils;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputRes>()
            .add_system_set(
                SystemSet::on_enter(GameState::PlayerInput)
                    .with_system(reset_input)
            )
            .add_system_set(
                SystemSet::on_update(GameState::PlayerInput)
                    // .with_system(mouse_input)
                    .with_system(keys)
            );
    }
}

fn reset_input(mut res: ResMut<InputRes>) {
    // res.mode = InputMode::TileShift;
}

fn keys(
    keys: ResMut<Input<KeyCode>>,
    mut res: ResMut<InputRes>,
    mut ev_command: EventWriter<CommandEvent>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for (key, dir) in KEY_MAPPING {
        if !keys.just_pressed(key) { continue; }
        let command = match res.mode {
            InputMode::TileShift => CommandType::TransformTiles(TileTransform::Shift(dir)),
            InputMode::TileSwitch => CommandType::TransformTiles(TileTransform::Switch(dir)),
            InputMode::TileRotate => {
                let clockwise = match key {
                    KeyCode::D => true,
                    KeyCode::A => false,
                    _ => continue
                };
                CommandType::TransformTiles(TileTransform::Rotate(clockwise))
            }
        };
        ev_command.send(CommandEvent(command));
        // only one command can be sent
        break;
    }
    if keys.just_pressed(KeyCode::Return) {
        ev_command.send(CommandEvent(CommandType::PlayerWait));
    }
    if keys.just_pressed(KeyCode::Space) {
        res.mode = match res.mode {
            InputMode::TileSwitch => InputMode::TileShift,
            InputMode::TileShift => InputMode::TileRotate,
            InputMode::TileRotate => InputMode::TileSwitch
        };
        ev_ui.send(ReloadUIEvent);
    }
    if keys.just_pressed(KeyCode::I) {
        res.extra_info = !res.extra_info;
        ev_ui.send(ReloadUIEvent);
    }
}

const KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
];

// fn mouse_input (
//     windows: Res<Windows>,
//     camera_query: Query<(&Camera, &GlobalTransform)>,
//     buttons: Res<Input<MouseButton>>,
//     mut res: ResMut<InputRes>,
//     mut ev_command: EventWriter<CommandEvent>,
//     mut ev_action: EventWriter<ActionEvent>
// ) {
//     if buttons.just_pressed(MouseButton::Left) {
//         if let Some(world_v) = utils::mouse_to_world(&windows, &camera_query) {
//             if let Some(v) = utils::world_to_tile_position(world_v) {
//                 match res.mode {
//                     InputMode::TileShift => match res.selected {
//                         Some(s) => {
//                             ev_command.send(
//                                 CommandEvent(CommandType::MapShift(v, s))
//                             );
//                             res.selected = None;
//                         },
//                         None => res.selected = Some(v)
//                     },
//                     InputMode::Place => {
//                         ev_action.send(ActionEvent(
//                             ActionKind::SpawnPiece(v, "Rock".to_string())
//                         ));
//                     }
//                 }

//             }
//         }
//     }
// }

#[derive(Debug)]
pub enum InputMode {
    TileShift,
    TileSwitch,
    TileRotate
}
impl InputMode {
    pub fn to_str(&self) -> &str {
        match self {
            Self::TileShift => "TileShift",
            Self::TileSwitch => "TileSwitch",
            Self::TileRotate => "TileRotate",
        }
    }
}
impl Default for InputMode {
    fn default() -> Self {
        InputMode::TileShift
    }
}

#[derive(Default, Resource)]
pub struct InputRes {
    pub selected: Option<Vector2Int>,
    pub mode: InputMode,
    pub extra_info: bool
}