use bevy::prelude::*;

#[derive(Component)]
pub struct PieceRenderer {
    pub target: Entity
}

#[derive(Component)]
pub struct ProjectileRenderer {
    pub target: Entity,
    pub linear_position: Vec3
}

#[derive(Component)]
pub struct TileRenderer {
    pub target: Entity
}