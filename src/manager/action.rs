use bevy::prelude::*;

use crate::pieces;
use crate::player;
use crate::states::GameState;
use crate::tiles;
use crate::units;

use super::{CommandEvent, CommandType};

pub fn update_units(
    mut commands: Commands,
    unit_query: Query<(Entity, &units::Unit)>
) {
    for (entity, unit) in unit_query.iter() {
        if unit.hp > 0 { continue; }

        commands.entity(entity)
            .despawn_recursive();
    }
}

pub fn piece_interaction(
    mut commands: Commands,
    player_query: Query<&player::Player>,
    piece_query: Query<
        (
            Entity,
            &Parent,
            Option<&pieces::Fixture>,
            Option<&pieces::Item>,
            Option<&pieces::Interactive>
        ), 
        (With<pieces::Piece>, Without<units::Unit>)
    >,
    tile_query: Query<&tiles::Tile>,
    mut res: ResMut<super::ManagerRes>,
    mut ev_command: EventWriter<CommandEvent>,
) {
    for (entity, parent, fixture, item, interactive) in piece_query.iter() {
        let player = player_query.get_single().unwrap();
        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v != player.v { continue; }

        if fixture.is_some() { res.is_descending = true; }
        if item.is_some() {
            commands.entity(entity)
                .despawn_recursive();
        }
        if let Some(i) = interactive {
            ev_command.send(CommandEvent(i.command));
        }
    }
}

pub fn heal_command(
    mut ev_command: EventReader<CommandEvent>,
    mut player_query: Query<(&player::Player, &mut units::Unit)>,
) {
    for ev in ev_command.iter() {
        if let CommandType::Heal(val) = ev.0 {
            let (_player, mut unit) = player_query.get_single_mut().unwrap();
            unit.add_hp(val);            
        }
    }
}