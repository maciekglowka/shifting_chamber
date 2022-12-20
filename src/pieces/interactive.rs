use bevy::prelude::*;

use crate::manager::CommandType;

#[derive(Component)]
pub struct Interactive {
    pub command: CommandType
}