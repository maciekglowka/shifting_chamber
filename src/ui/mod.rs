use bevy::prelude::*;

use crate::states::GameState;

mod cursor;
mod overlays;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(overlays::load_assets)
            .add_startup_system(cursor::load_assets)
            .add_system_set(SystemSet::on_enter(GameState::PlayerInput)
                .with_system(overlays::update_overlays)
            )
            .add_system_set(SystemSet::on_update(GameState::PlayerInput)
                .with_system(cursor::update_cursor)
            );  
    }  
}