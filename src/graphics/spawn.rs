use bevy::prelude::*;

use crate::pieces::components::Piece;

pub fn spawn_piece_renderer(
    piece_query: Query<(Entity, &Piece), Added<Piece>>
) {
    
}