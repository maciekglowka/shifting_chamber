use bevy::{
    ecs::system::EntityCommands,
    prelude::*
};
use serde::Deserialize;
use serde_yaml;
use serde_yaml::Mapping;
use std::{
    cmp::min,
    collections::HashMap
};

use crate::actions::{ ActionKind, DamageKind, StatKind};

// dynamic components, added in runtime - context depending
#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Poisoned {
    pub value: u32
}

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
pub struct Fixture;

#[derive(Component, Deserialize)]
pub struct Instant {
    // triggered automatically
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Interactive {
    // triggered manually, through ui - when stepped upon
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Item;

#[derive(Component, Deserialize)]
pub struct Manual {
    // triggered manually, through ui - when in the inventory
    pub kind: ActionKind
}

#[derive(Component, Deserialize)]
pub struct Poisonous {
    pub value: u32
}

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
    hp: u32,
    pub stats: HashMap<StatKind, u32>
}

impl Unit {
    pub fn new(stats: HashMap<StatKind, u32>) -> Unit {
        Unit { hp: stats[&StatKind::HP], stats}
    }
    pub fn hp(&self) -> u32 {
        self.hp
    }
    pub fn add_hp(&mut self, val: u32) {
        self.hp = min(
            self.stats[&StatKind::HP],
            self.hp + val
        );
    }
    pub fn sub_hp(&mut self, val: u32) {
        self.hp = self.hp.saturating_sub(val);
    }
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
        "Manual" => insert::<Manual>(ec, data),
        "Poisonous" => insert::<Poisonous>(ec, data),
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

pub fn get_effective_dmg(
    entity: Entity,
    unit: &Unit,
    damage_query: &Query<&Damage>,
    children: Option<&Children>
 ) -> (DamageKind, u32) {
    // TODO incl children (items)
    let dmg = match damage_query.get(entity) {
        Ok(d) => d,
        _ => return (DamageKind::None, 0)
    };
    let val = dmg.value + unit.stats.get(&StatKind::ST).unwrap_or(&0);
    (dmg.kind, val)
}

pub fn get_poisonous(
    entity: Entity,
    poisonous_query: &Query<&Poisonous>,
    children: Option<&Children>
 ) -> u32 {
    get_components_with_children(entity, poisonous_query, children)
        .iter()
        .map(|a| a.value)
        .sum()
}

pub fn get_components_with_children<'a, T: Component>(
    entity: Entity,
    query: &'a Query<&T>,
    children: Option<&Children>
) -> Vec<&'a T> {
    let mut v = match query.get(entity) {
        Ok(c) => vec!(c),
        _ => Vec::new()
    };
    if let Some(children) = children {
        for child in children.iter() {
            if let Ok(c) = query.get(*child) {
                v.push(c);
            }
        }
    }
    v
}