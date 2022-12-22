use bevy::prelude::*;
use rand::Rng;

use crate::actions::ActionKind;
use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::vectors::Vector2Int;

pub mod components;
mod renderer;
mod systems;


pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_exit(GameState::MapInit)
                    .with_system(furnish)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Action)
                    .with_system(systems::fights::check_fights)
                    .with_system(systems::interactions::check_interactions)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Action)
                    .with_system(systems::fights::kill_units)
                    .with_system(systems::items::pick_items)
            );
    }
}
 

// TO redo and move to systems
pub fn furnish(
    mut commands: Commands,
    tile_res: Res<TileRes>,
    assets: Res<renderer::PieceAssets>
) {
    let RESTRICTED: [Vector2Int; 6] = [
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2),
        Vector2Int::new(0, 0),
        Vector2Int::new(MAP_SIZE/2 - 1, MAP_SIZE/2),
        Vector2Int::new(MAP_SIZE/2 + 1, MAP_SIZE/2),
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2 - 1),
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2 + 1),
    ];

    let fixture = commands.spawn((
        components::Piece,
        components::Fixture,
        components::Interactive { 
            kind: ActionKind::Descend
        },
        renderer::get_piece_renderer(&assets.fixture_texture, 1)
    ))
    .id();
    commands.entity(tile_res.tiles[&Vector2Int::new(0, 0)])
        .push_children(&[fixture]);

    let mut rng = rand::thread_rng();

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            if RESTRICTED.contains(&v) { continue; }
            
            if rng.gen_bool(0.75) { continue; }

            if rng.gen_bool(0.75) {
                let piece = commands.spawn((
                        components::Damage { value: 2 },
                        components::Piece,
                        components::Unit::new(2),
                        renderer::get_piece_renderer(&assets.unit_texture, 1)
                    ))
                    .id();
                commands.entity(tile_res.tiles[&v])
                    .push_children(&[piece]);
            } else {
                let piece = commands.spawn((
                    components::Piece,
                    components::Item,
                    components::Interactive { 
                        kind: ActionKind::Heal(2) 
                    },
                    renderer::get_piece_renderer(&assets.item_texture, 0)
                ))
                .id();
                commands.entity(tile_res.tiles[&v])
                    .push_children(&[piece]);
            }
        }
    }
}