use bevy::prelude::*;
use rand::Rng;

use crate::data::DataAssets;
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
    assets: Res<renderer::PieceAssets>,
    data_assets: Res<DataAssets>
) {
    let RESTRICTED: [Vector2Int; 6] = [
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2),
        Vector2Int::new(0, 0),
        Vector2Int::new(MAP_SIZE/2 - 1, MAP_SIZE/2),
        Vector2Int::new(MAP_SIZE/2 + 1, MAP_SIZE/2),
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2 - 1),
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2 + 1),
    ];

    spawn_piece(&mut commands, "Stair".into(), Vector2Int::new(0, 0), &tile_res, &assets, &data_assets);

    let mut rng = rand::thread_rng();

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            if RESTRICTED.contains(&v) { continue; }
            
            if rng.gen_bool(0.75) { continue; }

            if rng.gen_bool(0.75) {
                spawn_piece(
                    &mut commands, 
                    "Face".into(),
                    v,
                    &tile_res,
                    &assets,
                    &data_assets
                );
            } else {
                spawn_piece(
                    &mut commands, 
                    "Heal".into(),
                    v,
                    &tile_res,
                    &assets,
                    &data_assets
                );
            }
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    name: String,
    v: Vector2Int,
    tile_res: &TileRes,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) {
    let err = &format!("Wrong data structure for {}", name);
    let data = data_assets.entities[&name].as_mapping().expect(err);
    let components = data["components"].as_mapping().expect(err);

    let mut piece = commands.spawn((
        components::Piece,
        renderer::get_piece_renderer(&data["sprite"], &assets)
    ));

    for (k, v) in components.iter() {
        components::insert_from_data(
            &mut piece, k.as_str().unwrap(), v.clone()
        ).unwrap();
    }

    let entity = piece.id();
    commands.entity(tile_res.tiles[&v])
        .push_children(&[entity]);
}
