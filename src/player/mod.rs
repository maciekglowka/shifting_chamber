use bevy::prelude::*;

use crate::states::GameState;
use crate::tiles::Tile;
use crate::units::Unit;
use crate::vectors::Vector2Int;

mod renderer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_enter(GameState::MapInit)
                    .with_system(spawn_player)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Action)
                    .with_system(handle_fights)
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
        Player { v: Vector2Int::new(0, 0) },
        Unit::new(),
        renderer::get_renderer(assets.as_ref())
    ));
}

fn handle_fights(
    mut player_query: Query<(&mut Unit, &Player)>,
    mut npc_query: Query<(&mut Unit, &Parent), Without<Player>>,
    tile_query: Query<&Tile>
) {
    for (mut npc, parent) in npc_query.iter_mut() {
        let (mut player_unit, player) = player_query.get_single_mut().unwrap();

        let tile = tile_query.get(parent.get()).unwrap();
        if tile.v.manhattan(player.v) > 1 { continue; }

        npc.hp = npc.hp.saturating_sub(player_unit.attack.value);
        player_unit.hp = player_unit.hp.saturating_sub(npc.attack.value);
    }
}