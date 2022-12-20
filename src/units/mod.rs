use bevy::prelude::*;

use crate::common::Attack;

#[derive(Component)]
pub struct Unit {
    pub hp: u32,
    pub attack: Attack
}

impl Unit {
    pub fn new(hp: u32) -> Unit {
        Unit { hp: hp, attack: Attack { value: 2 } }
    }
}