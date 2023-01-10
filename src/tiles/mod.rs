use bevy::prelude::*;
use std::collections::HashMap;

use crate::actions::{ActionKind, ActionEvent};
use crate::globals::MAP_SIZE;
use crate::states::GameState;
use crate::pieces::components::{Piece, Spawner, Unit};
use crate::vectors::Vector2Int;

mod renderer;

pub struct TileSwapEvent(Entity, Entity);

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRes>()
            .add_event::<TileSwapEvent>()
            .add_startup_system(renderer::load_assets)
            .add_system_set(
                SystemSet::on_enter(GameState::MapInit)
                    .with_system(spawn_map)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::PlayerInput)
                    .with_system(swap_action)
            );
    }
}

fn spawn_map(
    mut commands: Commands,
    mut res: ResMut<TileRes>,
    assets: Res<renderer::TileAssets>
) {
    clear_map(&mut commands, res.as_ref());

    let mut tiles = HashMap::new();
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((
                    Tile::new(v),
                    renderer::get_tile_renderer(v, assets.as_ref())
                ))
                .id();
            tiles.insert(v, tile);
        }
    }
    res.tiles = tiles;
}

fn clear_map(
    commands: &mut Commands,
    res: &TileRes
) {
    for entity in res.tiles.values() {
        commands.entity(*entity).despawn_recursive();
    }
}

pub fn shift_tiles(
    origin: Vector2Int,
    dir: Vector2Int,
    tile_query: &mut Query<&mut Tile>,
    res: &mut TileRes,
    ev_tile: &mut EventWriter<TileSwapEvent>
) {
    let (base, offset) = match dir {
        Vector2Int::LEFT | Vector2Int::RIGHT => {
            (Vector2Int::new(origin.x, 0), Vector2Int::new(0, 1))
        },
        Vector2Int::UP | Vector2Int::DOWN => {
            (Vector2Int::new(0, origin.y), Vector2Int::new(1, 0))
        },
        _ => return
    };

    for i in 0..MAP_SIZE {
        let v0 = base + i * offset;
        let v1 = v0 + dir;

        let e0 = res.tiles[&v0];
        let e1 = res.tiles[&v1];

        if let Ok(mut t0) = tile_query.get_mut(e0) { t0.v = v1; }
        if let Ok(mut t1) = tile_query.get_mut(e1) { t1.v = v0; }

        res.tiles.insert(v0, e1);
        res.tiles.insert(v1, e0);
        ev_tile.send(TileSwapEvent(e0, e1));
    }
}

fn swap_action(
    mut ev_command: EventReader<TileSwapEvent>,
    tile_query: Query<(Entity, Option<&Children>), With<Tile>>,
    piece_query: Query<(&Piece, Option<&Spawner>)>,
    mut ev_action: EventWriter<ActionEvent>
) {
    for ev in ev_command.iter() {
        for i in 0..2 {
            let (e0, e1) = match i {
                0 => (ev.0, ev.1),
                _ => (ev.1, ev.0)
            };
            let t0 = tile_query.get(e0).unwrap();
            let t1 = tile_query.get(e1).unwrap();
            
            if t1.1.iter().flat_map(|a| *a).any(|a| piece_query.get(*a).is_ok()) {
                // if other tile has any piece on it, we do not spawn
                continue;
            }
            for child in t0.1.iter().flat_map(|a| *a) {
                if let Ok((_piece, Some(spawner))) = piece_query.get(*child) {
                    ev_action.send(ActionEvent(ActionKind::SpawnPiece(t1.0, spawner.piece.clone())))
                }
            }
        }
    }
}

pub fn can_shift(
    origin: Vector2Int,
    dir: Vector2Int,
    player_v: Vector2Int,
    unit_query: &Query<&Parent, With<Unit>>,
    res: &TileRes
) -> bool {
    // TODO needs refactoring
    let v = match dir {
        Vector2Int::LEFT | Vector2Int::RIGHT => {
            match player_v.x {
                x if x == origin.x => Some(Vector2Int::new((origin+dir).x, player_v.y)),
                x if x == (origin+dir).x => Some(Vector2Int::new(origin.x, player_v.y)),
                _ => None
            }
        },
        Vector2Int::UP | Vector2Int::DOWN => {
            match player_v.y {
                y if y == origin.y => Some(Vector2Int::new(player_v.x, (origin+dir).y)),
                y if y == (origin+dir).y => Some(Vector2Int::new(player_v.x, origin.y)),
                _ => None
            }
        },
        _ => None
    };

    if v.is_none() { return true; }
    let tile = res.tiles[&v.unwrap()];

    for parent in unit_query.iter() {
        if parent.get() == tile {
            return false
        }
    }
    true
}

#[derive(Default, Resource)]
pub struct TileRes {
    pub tiles: HashMap<Vector2Int, Entity>
}

#[derive(Component)]
pub struct Tile {
    pub v: Vector2Int
}

impl Tile {
    fn new(v: Vector2Int) -> Tile {
        Tile { v }
    }
}