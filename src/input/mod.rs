use bevy::{
    input::{
        keyboard::KeyboardInput,
        touch::TouchPhase
    },
    prelude::*,
};

use crate::player::upgrades::TransformKind;
use crate::states::GameState;
use crate::manager::{CommandEvent, CommandType, GameRes};
use crate::ui::ReloadUIEvent;
use crate::vectors::{ORTHO_DIRECTIONS, Vector2Int};
use crate::tiles::transform::TileTransform;

mod utils;

const SWIPE_THRESH: f32 = 50.;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputRes>()
            .add_systems(OnEnter(GameState::GameInit), reset_input)
            .add_systems(Update, keys.run_if(in_state(GameState::PlayerInput)))
            .add_systems(Update, touches.run_if(in_state(GameState::PlayerInput)))
            // .add_systems(Update, keys_title.run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, keys_endgame.run_if(in_state(GameState::GameWin)));
    }
}

fn reset_input(mut res: ResMut<InputRes>) {
    res.mode = TransformKind::default();
}

fn any_input(
    key_ev: &mut EventReader<KeyboardInput>,
    touch_ev: &mut EventReader<TouchInput>,
) -> bool {
    for ev in key_ev.iter() {
        if let bevy::input::ButtonState::Released = ev.state {
            return true
        }
    }
    for ev in touch_ev.iter() {
        if let TouchPhase::Ended = ev.phase {
            return true
        }
    }
    false
}

// fn keys_title(
//     mut key_ev: EventReader<KeyboardInput>,
//     mut touch_ev: EventReader<TouchInput>,
//     mut ev_command: EventWriter<CommandEvent>
// ) {
//     if any_input(&mut key_ev, &mut touch_ev) {
//         ev_command.send(CommandEvent(CommandType::Start));
//     }
// }

fn keys_endgame(
    mut key_ev: EventReader<KeyboardInput>,
    mut touch_ev: EventReader<TouchInput>,
    mut ev_command: EventWriter<CommandEvent>
) {
    if any_input(&mut key_ev, &mut touch_ev) {
        ev_command.send(CommandEvent(CommandType::RestartGame));
    }
}

fn send_dir_action(
    dir: Vector2Int,
    ev_command: &mut EventWriter<CommandEvent>,
    res: &InputRes,
) {
    if !ORTHO_DIRECTIONS.contains(&dir) { return }
    let command = match res.mode {
        TransformKind::TileShift => CommandType::TransformTiles(TileTransform::Shift(dir)),
        TransformKind::TileSwitch => CommandType::TransformTiles(TileTransform::Switch(dir)),
        TransformKind::TileRotate => {
            let clockwise = match dir {
                Vector2Int::RIGHT => true,
                Vector2Int::LEFT => false,
                _ => return
            };
            CommandType::TransformTiles(TileTransform::Rotate(clockwise))
        }
    };
    ev_command.send(CommandEvent(command));
}

fn touches(
    mut touch_ev: EventReader<TouchInput>,
    mut res: ResMut<InputRes>,
    mut ev_command: EventWriter<CommandEvent>
) {
    for ev in touch_ev.iter() {
        match ev.phase {
            TouchPhase::Started => {
                res.swipe_start = Some(ev.position)
            },
            TouchPhase::Ended => {
                if let Some(start) = res.swipe_start {
                    let dx = match ev.position.x - start.x {
                        a if a > SWIPE_THRESH => 1,
                        a if a < -SWIPE_THRESH => -1,
                        _ => 0
                    };
                    let dy = match ev.position.y - start.y {
                        a if a > SWIPE_THRESH => -1,
                        a if a < -SWIPE_THRESH => 1,
                        _ => 0
                    };
                    send_dir_action(Vector2Int::new(dx, dy), &mut ev_command, res.as_ref());
                }
                res.swipe_start = None;
            }
            _ => {}
        }
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
        send_dir_action(dir, &mut ev_command, res.as_ref());
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

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 8] = [
    (KeyCode::W, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
    (KeyCode::Left, Vector2Int::LEFT), (KeyCode::Right, Vector2Int::RIGHT),
    (KeyCode::Up, Vector2Int::UP), (KeyCode::Down, Vector2Int::DOWN),
];
const DIGIT_KEYS: [(KeyCode, usize); 3] = [
    (KeyCode::Key1, 1), (KeyCode::Key2, 2), (KeyCode::Key3, 3)
];

#[derive(Default, Resource)]
pub struct InputRes {
    pub selected: Option<Vector2Int>,
    pub mode: TransformKind,
    pub show_help: bool,
    pub swipe_start: Option<Vec2>
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
