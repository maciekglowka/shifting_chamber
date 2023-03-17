use bevy::prelude::*;

use crate::player::upgrades::TransformKind;
use crate::states::GameState;
use crate::manager::{CommandEvent, CommandType, GameRes};
use crate::ui::ReloadUIEvent;
use crate::vectors::Vector2Int;
use crate::tiles::transform::TileTransform;

mod utils;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputRes>()
            .add_system(reset_input.in_schedule(OnEnter(GameState::GameInit)))
            .add_system(keys.in_set(OnUpdate(GameState::PlayerInput)))
            .add_system(keys_title.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(keys_endgame.in_set(OnUpdate(GameState::GameOver)))
            .add_system(keys_endgame.in_set(OnUpdate(GameState::GameWin)));
    }
}

fn reset_input(mut res: ResMut<InputRes>) {
    res.mode = TransformKind::default();
}

fn keys_title(
    keys: ResMut<Input<KeyCode>>,
    mut ev_command: EventWriter<CommandEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        ev_command.send(CommandEvent(CommandType::Start));
    }
}

fn keys_endgame(
    keys: ResMut<Input<KeyCode>>,
    mut ev_command: EventWriter<CommandEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        ev_command.send(CommandEvent(CommandType::Restart));
    }
}

fn keys(
    keys: ResMut<Input<KeyCode>>,
    mut res: ResMut<InputRes>,
    game_res: Res<crate::manager::GameRes>,
    mut ev_command: EventWriter<CommandEvent>,
    mut ev_ui: EventWriter<ReloadUIEvent>
) {
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) { continue; }
        let command = match res.mode {
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
    for (key, idx) in DIGIT_KEYS {
        if !keys.just_pressed(key) { continue; }
        res.set_mode_by_idx(idx, game_res.as_ref());
        ev_ui.send(ReloadUIEvent);
    }
    if keys.just_pressed(KeyCode::Space) {
        ev_command.send(CommandEvent(CommandType::PlayerWait));
    }
    if keys.just_pressed(KeyCode::H) {
        res.show_help = !res.show_help;
        ev_ui.send(ReloadUIEvent);
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
];
const DIGIT_KEYS: [(KeyCode, usize); 3] = [
    (KeyCode::Key1, 1), (KeyCode::Key2, 2), (KeyCode::Key3, 3)
];

#[derive(Default, Resource)]
pub struct InputRes {
    pub selected: Option<Vector2Int>,
    pub mode: TransformKind,
    pub show_help: bool
}
impl InputRes {
    pub fn set_mode_by_kind(&mut self, kind: TransformKind, game_res: &GameRes) {
        if game_res.tile_transforms[&kind] {
            self.mode = kind;
        }
    }
    pub fn set_mode_by_idx(&mut self, idx: usize, game_res: &GameRes) {
        let kind = InputRes::get_transform_by_idx(idx);
        self.set_mode_by_kind(kind, game_res);
    }
    pub fn get_transform_by_idx(idx: usize) -> TransformKind {
        match idx {
            1 => TransformKind::TileShift,
            2 => TransformKind::TileSwitch,
            3 => TransformKind::TileRotate,
            _ => panic!("Wrong tile transform mode!")
        }
    }
}
