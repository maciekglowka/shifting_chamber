use bevy::{
    ecs::system::EntityCommands,
    prelude::*
};
use serde::Deserialize;
use serde_yaml;
use serde_yaml::Mapping;
use std::cmp;

use crate::actions::DamageKind;
use crate::vectors::{Vector2Int, ORTHO_DIRECTIONS};

// marker / data components - added in game
#[derive(Component)]
pub struct Piece {
    pub name: String
}

#[derive(Component)]
pub struct Projectile {
    pub source: Vector2Int,
    pub target: Vector2Int
}

// serialized components - object data

#[derive(Clone, Component, Deserialize)]
pub struct Damage {
    pub value: u32,
    pub kind: DamageKind
}

#[derive(Clone, Component, Deserialize)]
pub struct Explosive;

#[derive(Component, Debug, Default, Deserialize)]
pub struct Health {
    #[serde(skip)]
    pub value: u32,
    pub max: u32
}
impl Health {
    pub fn add(&mut self, val: u32) {
        self.value = cmp::min(self.value + val, self.max);
    }
    pub fn sub(&mut self, val: u32) {
        self.value = self.value.saturating_sub(val);
    }
}

#[derive(Component, Deserialize)]
pub struct Occupier;

#[derive(Component, Deserialize)]
pub struct Range {
    pub fields: Vec<Vector2Int>
}

#[derive(Component, Deserialize)]
pub struct Walking {
    #[serde(skip)]
    pub planned_move: Option<Vector2Int>,
    #[serde(default="ortho_fields")]
    pub fields: Vec<Vector2Int>
}

fn ortho_fields() -> Vec<Vector2Int> {
    ORTHO_DIRECTIONS.into()
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
        "Damage" => insert::<Damage>(ec, data),
        "Explosive" => insert::<Explosive>(ec, data),
        "Health" => insert::<Health>(ec, data),
        "Occupier" => insert::<Occupier>(ec, data),
        "Range" => insert::<Range>(ec, data),
        "Walking" => insert::<Walking>(ec, data),
        _ => ()
    };        
}

fn insert<T>(ec: &mut EntityCommands, data: serde_yaml::Value)
where for<'de> T: Bundle + Deserialize<'de>
{
    let mut component = serde_yaml::from_value::<T>(data).expect("Wrong component list!");
    ec.insert(component);
}
