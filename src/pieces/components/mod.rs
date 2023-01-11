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
pub struct Instant {
    // triggered automatically
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Interactive {
    // triggered manually, through ui
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Item;

#[derive(Component, Deserialize)]
pub struct Protect {
    pub value: u32,
    pub kind: DamageKind
}

#[derive(Component, Debug, Deserialize)]
pub struct Spawner {
    pub piece: String
}

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
        insert_by_name(
            ec, k.as_str().unwrap(), v.clone()
        );
    }
}

fn insert_by_name(ec: &mut EntityCommands, name: &str, data: serde_yaml::Value) {
    match name {
        "Collectable" => insert::<Collectable>(ec, data),
        "Damage" => insert::<Damage>(ec, data),
        "Effect" => insert::<Effect>(ec, data),
        "Fixture" => insert::<Fixture>(ec, data),
        "Instant" =>  insert::<Instant>(ec, data),
        "Interactive" =>  insert::<Interactive>(ec, data),
        "Item" => insert::<Item>(ec, data),
        "Protect" => insert::<Protect>(ec, data),
        "Spawner" => insert::<Spawner>(ec, data),
        "Temporary" => insert::<Temporary>(ec, data),
        "Unit" => insert::<Unit>(ec, data),
        _ => ()
    };        
}

fn insert<T>(ec: &mut EntityCommands, data: serde_yaml::Value)
where for<'de> T: Bundle + Deserialize<'de>
{
    ec.insert(serde_yaml::from_value::<T>(data).expect("Wrong component list!"));
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