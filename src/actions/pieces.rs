use bevy::prelude::*;

use crate::data::DataAssets;
use crate::pieces::{
    components::{Protect, Unit},
    renderer::PieceAssets,
    spawn_piece_at_entity
};

use super::{ActionEvent, ActionKind};

pub fn spawn_piece(
    mut ev_action: EventReader<ActionEvent>,
    mut commands: Commands,
    piece_assets: Res<PieceAssets>,
    data_assets: Res<DataAssets>
) {
    for ev in ev_action.iter() {
        if let ActionKind::SpawnPiece(entity, name) = ev.0.clone() {
            spawn_piece_at_entity(
                &mut commands,
                name,
                entity,
                piece_assets.as_ref(),
                data_assets.as_ref()
            )
        }
    }
}
