use bevy::prelude::*;
use std::cmp::min;

use crate::pieces::components::{Piece, Unit};
use crate::player::Player;

use super::{ActionEvent, ActionKind};

pub fn heal(
    mut player_query: Query<&mut Unit, With<Player>>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Heal(val) = ev.0 {
            if let Ok(mut unit) = player_query.get_single_mut() {
                unit.hp = min(unit.max_hp, unit.hp + val);
            }
        }
    }
}

pub fn descend(
    mut player_query: Query<&mut Player>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Descend = ev.0 {
            if let Ok(mut player) = player_query.get_single_mut() {
                player.is_descending = true;
            }
        }
    }
}

pub fn score(
    mut game_res: ResMut<crate::manager::GameRes>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Score(val) = ev.0 {
            game_res.score += val;
        }
    }
}

pub fn pick_item(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<&Parent>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Pick(entity) = ev.0 {
            let player_entity = player_query.get_single().unwrap();
            let parent = item_query.get(entity).unwrap();

            commands.entity(parent.get())
                .remove_children(&[entity]);
            commands.entity(entity)
                .remove::<SpriteSheetBundle>()
                .remove::<Piece>();
            commands.entity(player_entity)
                .push_children(&[entity]);

        }
    }
}
