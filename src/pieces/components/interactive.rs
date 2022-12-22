use bevy::prelude::*;

use crate::actions::ActionKind;

#[derive(Component)]
pub struct Interactive {
    pub kind: ActionKind
}