use bevy::prelude::*;
use std::cmp;

use crate::actions::{ActionEvent, HealAction, IncreaseHPAction, StartMapAction};
use crate::globals::UPGRADE_PENALTY;
use crate::pieces::components;
use crate::player::{
    Player,
    upgrades::UpgradeKind
};
use crate::states::GameState;
use crate::tiles;
use crate::ui;

use super::{CommandEvent, CommandType, GameRes};

// all those systems are invoked by a direct player command
// they should end player_input state on success

pub fn wait(
    mut ev_command: EventReader<CommandEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>
) {
    for ev in ev_command.iter() {
        if let CommandType::PlayerWait = ev.0 {
            next_state.set(GameState::NPCAction);
        }
    }
}

pub fn transform_tiles(
    mut ev_command: EventReader<CommandEvent>,
    player_query: Query<&Parent, With<Player>>,
    mut tile_query: Query<&mut tiles::Tile>,
    mut tile_res: ResMut<tiles::TileRes>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    mut ev_game: EventWriter<super::GameEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::TransformTiles(transform) = ev.0 {
            let player_v = match player_query.get_single() {
                Err(_) => continue,
                Ok(parent) => {
                    let Ok(tile) = tile_query.get(parent.get()) else { continue };
                    tile.v
                }
            };
            if tiles::transform::can_transform(transform, player_v, tile_res.as_ref()) {
                tiles::transform::execute(
                    transform,
                    player_v,
                    &mut tile_query,
                    tile_res.as_mut()
                );
                next_state.set(GameState::TileShift);
                ev_game.send(super::GameEvent(super::GameEventKind::TileTransformed));
            } else {
                ev_game.send(super::GameEvent(super::GameEventKind::WrongAction));
            }
        }
    }
}

pub fn upgrade(
    mut ev_command: EventReader<CommandEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    player_query: Query<Entity, With<Player>>,
    mut res: ResMut<GameRes>,
    mut ev_action: EventWriter<ActionEvent>
) {
    for ev in ev_command.iter() {
        if let CommandType::Upgrade(kind) = ev.0 {
            let Ok(player) = player_query.get_single() else { return };
            if kind != UpgradeKind::Skip {
                res.score -= UPGRADE_PENALTY;
            }
            match kind {
                // UpgradeKind::HealPlayer => ev_action.send(ActionEvent(ActionKind::Heal(player, 3))),
                UpgradeKind::HealPlayer => ev_action.send(
                    ActionEvent(Box::new(HealAction{ entity: player, value: 3 }))
                ),
                UpgradeKind::IncreaseAP => res.max_ap += 1,
                // UpgradeKind::IncreaseHP => ev_action.send(ActionEvent(ActionKind::IncreaseHP(player, 1))),
                UpgradeKind::IncreaseHP => ev_action.send(
                    ActionEvent(Box::new(IncreaseHPAction{ entity: player, value: 1 }))
                ),
                UpgradeKind::TileTransform(t) => { res.tile_transforms.insert(t, true); },
                UpgradeKind::Skip => ()
            };
            if kind.is_single() {
                res.possible_upgrades.remove(&kind);
            }
            ev_action.send(
                ActionEvent(Box::new(StartMapAction))
            );
        }
    }
}