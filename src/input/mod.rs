use bevy::prelude::*;

use crate::player::upgrades::TransformKind;
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
            .add_system(reset_input.in_schedule(OnEnter(GameState::PlayerInput)))
            .add_system(keys.in_set(OnUpdate(GameState::PlayerInput)));
    }
}

fn reset_input(mut res: ResMut<InputRes>) {
    // res.mode = InputMode::TileShift;
}

fn keys(
    keys: ResMut<Input<KeyCode>>,
    mut res: ResMut<InputRes>,
    game_res: Res<crate::manager::GameRes>,
    mut ev_command: EventWriter<CommandEvent>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for (key, dir) in KEY_MAPPING {
        if !keys.just_pressed(key) { continue; }
        let mode = &game_res.available_transforms[res.mode];
        let command = match mode {
            TransformKind::TileShift => CommandType::TransformTiles(TileTransform::Shift(dir)),
            TransformKind::TileSwitch => CommandType::TransformTiles(TileTransform::Switch(dir)),
            TransformKind::TileRotate => {
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
    if keys.just_pressed(KeyCode::Space) {
        ev_command.send(CommandEvent(CommandType::PlayerWait));
    }
    if keys.just_pressed(KeyCode::Return) {
        res.mode += 1;
        if res.mode > game_res.available_transforms.len() - 1 { res.mode = 0 }
        ev_ui.send(ReloadUIEvent);
    }
}

const KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
];

#[derive(Default, Resource)]
pub struct InputRes {
    pub selected: Option<Vector2Int>,
    pub mode: usize
}
