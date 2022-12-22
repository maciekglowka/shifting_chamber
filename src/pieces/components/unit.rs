use bevy::prelude::*;

#[derive(Component)]
pub struct Unit {
    pub hp: u32,
    pub max_hp: u32
}

impl Unit {
    pub fn new(hp: u32) -> Unit {
        Unit { 
            hp: hp,
            max_hp: hp
        }
    }
}