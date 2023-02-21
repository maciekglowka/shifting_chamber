use bevy::prelude::*;

use crate::data::DataAssets;
use crate::pieces::spawn_piece_at_v;
use crate::tiles::TileRes;

use super::{ActionEvent, ActionKind};

pub fn spawn_piece(
    mut ev_action: EventReader<ActionEvent>,
    mut commands: Commands,
    data_assets: Res<DataAssets>,
    tile_res: Res<TileRes>
) {
    // for ev in ev_action.iter() {
    //     if let ActionKind::SpawnPiece(v, name) = ev.0.clone() {
    //         spawn_piece_at_v(
    //             &mut commands,
    //             name,
    //             v,
    //             tile_res.as_ref(),
    //             data_assets.as_ref()
    //         );
    //     }
    // }
}