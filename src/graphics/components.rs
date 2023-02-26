use bevy::prelude::*;

#[derive(Component)]
pub struct PieceRenderer {
    pub target: Entity
}

#[derive(Component)]
pub struct TileRenderer {
    pub target: Entity
}