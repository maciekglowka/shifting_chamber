use bevy::prelude::*;

use crate::actions::DamageKind;
use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::pieces::components::{Damage, Unit};
use crate::vectors::Vector2Int;

mod renderer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_enter(GameState::GameInit)
                    .with_system(spawn_player)
            );
    }
}

#[derive(Component)]
pub struct Player {
    pub v: Vector2Int
}

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<renderer::PlayerAssets>
) {
    commands.spawn((
        Player { 
            v: Vector2Int::new(MAP_SIZE / 2, MAP_SIZE / 2)
        },
        Damage { value: 2, kind: DamageKind::Hit },
        Unit { hp: 3, max_hp: 3 },
        renderer::get_renderer(assets.as_ref())
    ));
}
