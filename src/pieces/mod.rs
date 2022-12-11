use bevy::prelude::*;
use rand::Rng;

use crate::fixtures::Fixture;
use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::tiles::TileRes;
use crate::units::Unit;
use crate::vectors::Vector2Int;

mod renderer;

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
        Fixture::new(),
        renderer::get_piece_renderer(assets.as_ref(), 4)
    ))
    .id();
    commands.entity(tile_res.tiles[&Vector2Int::new(0, 0)])
        .push_children(&[fixture]);

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            if RESTRICTED.contains(&v) { continue; }
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.75) { continue; }

            let piece = commands.spawn((
                    Unit::new(),
                    renderer::get_piece_renderer(assets.as_ref(), 1)
                ))
                .id();
            commands.entity(tile_res.tiles[&v])
                .push_children(&[piece]);
        }
    }
}
