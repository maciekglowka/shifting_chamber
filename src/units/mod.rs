use bevy::prelude::*;
use std::cmp::min;

use crate::common::Attack;

#[derive(Component)]
pub struct Unit {
    pub hp: u32,
    pub max_hp: u32,
    pub attack: Attack
}

impl Unit {
    pub fn new(hp: u32) -> Unit {
        Unit { 
            hp: hp,
            max_hp: hp,
            attack: Attack { value: 2 } 
        }
    }
    pub fn add_hp(&mut self, val: u32) {
        self.hp = min(self.hp + val, self.max_hp);
    }
}