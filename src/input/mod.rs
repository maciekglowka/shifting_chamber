use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::states::GameState;
use crate::manager::{CommandEvent, CommandType};
use crate::vectors::Vector2Int;

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
    mut ev_command: EventWriter<CommandEvent>
) {
    for (key, dir) in KEY_MAPPING {
        if !keys.just_pressed(key) { continue; }
        let command = match res.mode {
            InputMode::TileShift => CommandType::ShiftTiles(dir),
            InputMode::TileSwitch => CommandType::SwitchTiles(dir),
            InputMode::Punch => CommandType::Punch(dir),
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
            InputMode::TileShift => InputMode::TileSwitch,
            InputMode::Punch => InputMode::Punch
        };
        info!("Mode: {:?} set", res.mode);
        // // changing mode takes a turn as well
        // ev_command.send(CommandEvent(CommandType::PlayerWait));
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
    Punch
}
impl Default for InputMode {
    fn default() -> Self {
        InputMode::TileShift
    }
}

#[derive(Default, Resource)]
pub struct InputRes {
    pub selected: Option<Vector2Int>,
    pub mode: InputMode
}