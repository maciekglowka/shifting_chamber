use bevy::{
    ecs::system::EntityCommands,
    prelude::*
};
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;

use crate::actions::ActionKind;
use crate::actions::DamageKind;

// dynamic components, added in runtime - context depending
#[derive(Component)]
pub struct Piece;

// #[derive(Component)]
// pub struct Inventory;

// serialized components - object data

#[derive(Component, Deserialize)]
pub struct Collectable;

#[derive(Component, Deserialize)]
pub struct Damage {
    pub value: u32,
    pub kind: DamageKind
}

#[derive(Component, Deserialize)]
pub struct Fixture {}

#[derive(Component, Deserialize)]
pub struct Interactive {
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Item {}

#[derive(Component, Deserialize)]
pub struct Protect {
    pub value: u32,
    pub kind: DamageKind
}

// #[derive(Component)]
// pub struct Spreading;

#[derive(Component, Deserialize)]
pub struct Temporary {
    pub value: u32
}

#[derive(Component, Deserialize)]
pub struct Unit {
    pub hp: u32,
    pub max_hp: u32
}

pub fn insert_from_data(ec: &mut EntityCommands, name: &str, data: serde_yaml::Value) -> Result<(), serde_yaml::Error> {
    match name {
        "Collectable" => ec.insert(serde_yaml::from_value::<Collectable>(data)?),
        "Damage" => ec.insert(serde_yaml::from_value::<Damage>(data)?),
        "Fixture" => ec.insert(serde_yaml::from_value::<Fixture>(data)?),
        "Interactive" =>  ec.insert(serde_yaml::from_value::<Interactive>(data)?),
        "Item" => ec.insert(serde_yaml::from_value::<Item>(data)?),
        "Protect" => ec.insert(serde_yaml::from_value::<Protect>(data)?),
        "Temporary" => ec.insert(serde_yaml::from_value::<Temporary>(data)?),
        "Unit" => ec.insert(serde_yaml::from_value::<Unit>(data)?),
        _ => ec
    };        
    Ok(())
}
