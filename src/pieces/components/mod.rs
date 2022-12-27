use bevy::{
    ecs::system::EntityCommands,
    prelude::*
};
use serde::Deserialize;
use serde_yaml;

use crate::actions::ActionKind;

#[derive(Component, Deserialize)]
pub struct Damage {
    pub value: u32
}

#[derive(Component, Deserialize)]
pub struct Fixture {}

#[derive(Component, Deserialize)]
pub struct Interactive {
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Item {}

#[derive(Component)]
pub struct Piece;

// #[derive(Component)]
// pub struct Spreading;

#[derive(Component, Deserialize)]
pub struct Unit {
    pub hp: u32,
    pub max_hp: u32
}

pub fn insert_from_data(ec: &mut EntityCommands, name: &str, data: serde_yaml::Value) -> Result<(), serde_yaml::Error> {
    match name {
        "Damage" => ec.insert(serde_yaml::from_value::<Damage>(data)?),
        "Fixture" => ec.insert(serde_yaml::from_value::<Fixture>(data)?),
        "Interactive" =>  ec.insert(serde_yaml::from_value::<Interactive>(data)?),
        "Item" => ec.insert(serde_yaml::from_value::<Item>(data)?),
        "Unit" => ec.insert(serde_yaml::from_value::<Unit>(data)?),
        _ => ec
    };        
    Ok(())
}