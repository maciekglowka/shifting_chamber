use bevy::prelude::*;

use crate::common::Attack;

#[derive(Component)]
pub struct Unit {
    pub hp: u32,
    pub attack: Attack
}

impl Unit {
    pub fn new() -> Unit {
        Unit { hp: 5, attack: Attack { value: 2 } }
    }
}