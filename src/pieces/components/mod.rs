use bevy::prelude::*;

use crate::actions::ActionKind;

#[derive(Component)]
pub struct Damage {
    pub value: u32
}

#[derive(Component)]
pub struct Fixture;

#[derive(Component)]
pub struct Interactive {
    pub kind: ActionKind
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Unit {
    pub hp: u32,
    pub max_hp: u32
}

impl Unit {
    pub fn new(hp: u32) -> Unit {
        Unit { hp: hp, max_hp: hp }
    }
}