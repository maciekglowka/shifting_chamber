use bevy::prelude::*;

use crate::data::DataAssets;
use crate::manager::{CommandEvent, CommandType};
use crate::pieces::{
    components,
    components::{Piece, Unit}
};
use crate::player::Player;

use super::{ActionEvent, ActionKind};

pub fn heal(
    mut player_query: Query<&mut Unit, With<Player>>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Heal(val) = ev.0 {
            if let Ok(mut unit) = player_query.get_single_mut() {
                unit.add_hp(val);
            }
        }
    }
}

pub fn stat_upgrade(
    mut player_query: Query<&mut Unit, With<Player>>,
    mut ev_action: EventReader<ActionEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::StatUpgrade(kind, val) = ev.0 {
            if let Ok(mut unit) = player_query.get_single_mut() {
                let stat = val + unit.stats.get(&kind).unwrap_or(&0);
                unit.stats.insert(kind, stat);

                if kind == super::StatKind::HP { unit.add_hp(val); }
            }
        }
    }
}

pub fn descend(
    mut ev_action: EventReader<ActionEvent>,
    mut ev_command: EventWriter<CommandEvent>

) {
    for ev in ev_action.iter() {
        if let ActionKind::Descend = ev.0 {
            ev_command.send(CommandEvent(CommandType::NextLevel));
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

pub fn apply_effect(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut ev_action: EventReader<ActionEvent>,
    data_assets: Res<DataAssets>
) {
    for ev in ev_action.iter() {
        if let ActionKind::ApplyEffect(name) = &ev.0 {
            if let Ok(player_entity) = player_query.get_single() {
                let data_item = &data_assets.pieces[name];
    
                commands.entity(player_entity)
                    .with_children(|parent| {
                        let mut effect = parent.spawn_empty();
                        components::insert_from_list(&mut effect, &data_item.components);
                    });
            }
        }
    }
}

pub fn pick_item(
    mut commands: Commands,
    mut ev_action: EventReader<ActionEvent>,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<&Parent>,
) {
    for ev in ev_action.iter() {
        if let ActionKind::PickItem(entity) = &ev.0 {
            let player_entity = player_query.get_single().unwrap();
            let parent = item_query.get(*entity).unwrap();

            commands.entity(parent.get())
                .remove_children(&[*entity]);
            commands.entity(*entity)
                .remove::<SpriteSheetBundle>()
                .remove::<Piece>();
            commands.entity(player_entity)
                .push_children(&[*entity]);

        }
    }
}