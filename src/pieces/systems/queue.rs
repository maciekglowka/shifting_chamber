use bevy::prelude::*;

use crate::manager::{CommandEvent, CommandType};

use super::super::components::{Walking, Range};
use super::super::PieceRes;


pub fn plan_queue(
    mut piece_res: ResMut<PieceRes>,
    walking_query: Query<Entity, With<Walking>>,
    range_query: Query<Entity, With<Range>>
) {
    let queue = walking_query.iter()
        .chain(range_query.iter())
        .collect();

    piece_res.action_queue = queue;
}

pub fn update_queue(
    mut piece_res: ResMut<PieceRes>,
    mut ev_command: EventWriter<CommandEvent>
) {
    piece_res.action_queue.pop_front();
    if piece_res.action_queue.is_empty() {
        ev_command.send(CommandEvent(CommandType::TurnEnd));
    }
}