use bevy::{
    ecs::system::EntityCommands,
    prelude::*
};
use serde::Deserialize;
use serde_yaml;
use serde_yaml::Mapping;

use crate::actions::{ ActionKind, DamageKind};
use crate::data::DataAssets;

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
pub struct Effect;

#[derive(Component, Deserialize)]
pub struct Fixture {}

#[derive(Component, Deserialize)]
pub struct Interactive {
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Item;

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

pub fn insert_from_list(ec: &mut EntityCommands, component_list: &Mapping) {
    for (k, v) in component_list.iter() {
        insert_single(
            ec, k.as_str().unwrap(), v.clone()
        ).expect("Wrong component list!");
    }
}

fn insert_single(ec: &mut EntityCommands, name: &str, data: serde_yaml::Value) -> Result<(), serde_yaml::Error> {
    match name {
        "Collectable" => ec.insert(serde_yaml::from_value::<Collectable>(data)?),
        "Damage" => ec.insert(serde_yaml::from_value::<Damage>(data)?),
        "Effect" => ec.insert(serde_yaml::from_value::<Effect>(data)?),
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

pub fn get_piece_data<'a>(
    name: &'a str,
    data_assets: &'a DataAssets
) -> (&'a Mapping, &'a Mapping) {
    let err = &format!("Wrong data structure for {}", name);
    let data = data_assets.entities[name].as_mapping().expect(err);
    let components = data["components"].as_mapping().expect(err);
    return (data, components)
}