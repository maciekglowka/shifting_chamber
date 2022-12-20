use bevy::prelude::*;
use rand::Rng;

use crate::globals::MAP_SIZE;
use crate::manager;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::units::Unit;
use crate::vectors::Vector2Int;

mod fixtures;
mod interactive;
mod items;
mod renderer;

pub use fixtures::Fixture;
pub use interactive::Interactive;
pub use items::Item;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_exit(GameState::MapInit)
                    .with_system(furnish)
            );
    }
}
 
pub fn furnish(
    mut commands: Commands,
    tile_res: Res<TileRes>,
    // tile_query: Query<(Entity, &Tile, &Parent)>,
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
        Piece,
        Fixture::new(),
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
                        Piece,
                        Unit::new(2),
                        renderer::get_piece_renderer(&assets.unit_texture, 1)
                    ))
                    .id();
                commands.entity(tile_res.tiles[&v])
                    .push_children(&[piece]);
            } else {
                let piece = commands.spawn((
                    Piece,
                    Item,
                    Interactive { command: manager::CommandType::Heal(2) },
                    renderer::get_piece_renderer(&assets.item_texture, 0)
                ))
                .id();
                commands.entity(tile_res.tiles[&v])
                    .push_children(&[piece]);
            }
        }
    }
}


#[derive(Component)]
pub struct Piece;