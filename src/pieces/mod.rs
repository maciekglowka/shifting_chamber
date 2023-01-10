use bevy::prelude::*;
use rand::Rng;

use crate::actions::ActionKind;
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
                SystemSet::on_enter(GameState::PlayerInput)
                    .with_system(systems::items::examine_pickable_items)
                    .label("action")
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Action)
                    .with_system(systems::fights::check_fights)
                    .with_system(systems::interactions::check_interactions)
                    .with_system(systems::interactions::update_temporary)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Action)
                    .with_system(systems::fights::kill_units)
                    .with_system(systems::items::remove_disposable_items)
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
        Vector2Int::new(MAP_SIZE/2 - 1, MAP_SIZE/2),
        Vector2Int::new(MAP_SIZE/2 + 1, MAP_SIZE/2),
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2 - 1),
        Vector2Int::new(MAP_SIZE/2, MAP_SIZE/2 + 1),
        Vector2Int::new(0, 0),
    ];

    let ITEM: [Vector2Int; 3] = [
        Vector2Int::new(MAP_SIZE-1, 0),
        Vector2Int::new(0, MAP_SIZE-1),
        Vector2Int::new(MAP_SIZE-1, MAP_SIZE-1),
    ];

    spawn_piece_at_v(&mut commands, "Stair".into(), Vector2Int::new(0, 0), &tile_res, &assets, &data_assets);

    let mut rng = rand::thread_rng();

    for v in ITEM {
        if rng.gen_bool(0.5) { continue; }

        let name = match rng.gen_bool(0.5) {
            true => "Shield",
            false => "Heal"
        };

        spawn_piece_at_v(
            &mut commands, 
            name.into(),
            v,
            &tile_res,
            &assets,
            &data_assets
        );
    }

    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            if RESTRICTED.contains(&v) { continue; }
            if ITEM.contains(&v) { continue; }
            
            if rng.gen_bool(0.75) { continue; }

            spawn_piece_at_v(
                &mut commands, 
                "Face".into(),
                v,
                &tile_res,
                &assets,
                &data_assets
            );
        }
    }
}

fn spawn_piece_at_parent(
    commands: &mut Commands,
    name: String,
    parent: &Parent,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) {
    let entity = get_new_piece(commands, name, assets, data_assets);
    commands.entity(parent.get())
        .push_children(&[entity]);
}

fn spawn_piece_at_v(
    commands: &mut Commands,
    name: String,
    v: Vector2Int,
    tile_res: &TileRes,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) {
    let entity = get_new_piece(commands, name, assets, data_assets);
    commands.entity(tile_res.tiles[&v])
        .push_children(&[entity]);
}

fn get_new_piece(
    commands: &mut Commands,
    name: String,
    assets: &renderer::PieceAssets,
    data_assets: &DataAssets
) -> Entity {
    let (data, component_list) = components::get_piece_data(&name, data_assets);
    
    let mut piece = commands.spawn((
        components::Piece,
        renderer::get_piece_renderer(&data["sprite"], &assets)
    ));
    info!("{:?}", component_list.keys().map(|k| k.as_str()).collect::<Vec<_>>());
    if component_list.contains_key("Effect") {
        // when spawning an effect, wrap it inside interactive item
        piece.insert((
            components::Item,
            components::Interactive {
                kind: ActionKind::ApplyEffect(name)
            }
        ));
    } else {
        // otherwise just build component list normally
        components::insert_from_list(&mut piece, component_list);
    }   
    piece.id()
}
