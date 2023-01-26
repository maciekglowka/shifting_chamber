use bevy::prelude::*;
use serde::Deserialize;

use super::{DataAssets, YamlAsset};

const LEVEL_FILE: &str = "data_levels.yaml";

#[derive(Deserialize)]
pub struct LevelData {
    pub initial_points: i32,
    pub required_pieces: Vec<String>,
    pub extra_items: (usize, usize),
    pub extra_features: (usize, usize),
}

pub fn parse_data(
    mut assets: ResMut<DataAssets>,
    yaml_assets: Res<Assets<YamlAsset>>
) {
    let asset = yaml_assets.get(&assets.raw_levels).expect("No such asset!");
    let data: serde_yaml::Value = serde_yaml::from_str(&asset.value)
        .expect("Incorrect yaml!");
    for (k, v) in data.as_mapping().expect("Incorrect data format!") {
        let key: String = k.as_str().expect("Keys must be strings!").into();
        assets.levels.insert(
            key.clone(),
            serde_yaml::from_value(v.clone()).expect("Wrong data item!")
        );
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut assets: ResMut<DataAssets>
) {
    let data: Handle<YamlAsset> = asset_server.load(LEVEL_FILE);
    asset_list.0.push(data.clone_untyped());
    assets.raw_levels = data;
}
