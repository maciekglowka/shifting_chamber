use bevy::prelude::*;

use crate::pieces::components::{
    Health
};
use crate::tiles::Tile;
use crate::ui::BubbleEvent;

use super::{ActionEvent, ActionKind};

pub fn receive_damage(
    mut health_query: Query<(&mut Health, &Parent)>,
    tile_query: Query<&Tile>,
    mut ev_action: EventReader<ActionEvent>,
    mut ev_bubble: EventWriter<BubbleEvent>
) {
    for ev in ev_action.iter() {
        if let ActionKind::Damage(entity, _kind, value) = ev.0 {
            let Ok((mut health, parent)) = health_query.get_mut(entity) else { continue };
            health.sub(value);
            if let Ok(tile) = tile_query.get(parent.get()) {
                ev_bubble.send(BubbleEvent(tile.v, format!("-{}", value)));
            }
        }
    }
}
