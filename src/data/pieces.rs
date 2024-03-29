use bevy::prelude::*;
use serde::Deserialize;


use super::{DataAssets, YamlAsset};

const PIECE_FILES: [&str; 3] = ["data_fixtures.yaml", "data_units.yaml", "data_player.yaml"];

#[derive(Deserialize)]
pub enum SpriteColumns {
    Frames(usize),
    Variants(usize)
}

#[derive(Deserialize)]
pub struct SpriteData {
    pub atlas: String,
    pub index: usize,
    pub columns: Option<SpriteColumns>
}

#[derive(Deserialize)]
pub struct PieceData {
    pub min_level: Option<u32>,
    pub points: Option<i32>,
    pub sprite: SpriteData,
    pub components: serde_yaml::Mapping
}

pub fn parse_data(
    mut assets: ResMut<DataAssets>,
    yaml_assets: Res<Assets<YamlAsset>>
) {
    assets.unit_names = Vec::new();
    assets.fixture_names = Vec::new();

    for (name, piece) in assets.raw_pieces.clone() {
        let asset = yaml_assets.get(&piece).expect("No such asset!");
        let data: serde_yaml::Value = serde_yaml::from_str(&asset.value)
            .expect("Incorrect yaml!");
        for (k, v) in data.as_mapping().expect("Incorrect data format!") {
            let key: String = k.as_str().expect("Keys must be strings!").into();
            assets.pieces.insert(
                key.clone(),
                serde_yaml::from_value(v.clone()).expect(&format!("Wrong data item {:?}!", k))
            );
            match name.as_str() {
                "data_units" => assets.unit_names.push(key),
                "data_fixtures" => assets.fixture_names.push(key),
                _ => ()
            }
        }
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut assets: ResMut<DataAssets>
) {
    for fname in PIECE_FILES {
        let data: Handle<YamlAsset> = asset_server.load(fname);
        asset_list.0.push(data.clone_untyped());
        let parts = fname.split('.').collect::<Vec<_>>();
        assets.raw_pieces.push((parts[0].to_string(), data));
    }
}