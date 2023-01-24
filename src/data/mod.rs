use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture
};
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;

use crate::states::GameState;

const YML_FILES: [&str; 4] = [
    "data_effects.yaml",
    "data_fixtures.yaml", "data_items.yaml", "data_units.yaml"
];

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DataAssets>()
            .add_asset::<YamlAsset>()
            .init_asset_loader::<YamlAssetLoader>()
            .add_startup_system(load_assets)
            .add_system_set(
                SystemSet::on_exit(GameState::LoadAssets)
                    .with_system(parse_data)
            );
    }
}

fn parse_data(
    mut assets: ResMut<DataAssets>,
    yaml_assets: Res<Assets<YamlAsset>>
) {
    for entity in assets.raw_entities.clone() {
        let asset = yaml_assets.get(&entity).expect("No such asset!");
        let data: serde_yaml::Value = serde_yaml::from_str(&asset.value)
            .expect("Incorrect yaml!");
        for (k, v) in data.as_mapping().expect("Incorrect data format!") {
            let key = k.as_str().expect("Keys must be strings!").into();
            assets.entities.insert(
                key,
                serde_yaml::from_value(v.clone()).expect("Wrong data item!")
            );
        }
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut assets: ResMut<DataAssets>
) {
    for fname in YML_FILES {
        let data: Handle<YamlAsset> = asset_server.load(fname);
        asset_list.0.push(data.clone_untyped());
        assets.raw_entities.push(data);
    }
}

#[derive(Default, Resource)]
pub struct DataAssets {
    pub entities: HashMap<String, DataItem>,
    pub raw_entities: Vec<Handle<YamlAsset>>,
}

#[derive(Deserialize)]
pub struct DataItem {
    pub min_level: Option<u32>,
    pub points: Option<i32>,
    pub sprite: (String, usize),
    pub components: serde_yaml::Mapping
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "d1228bd6-057b-4b93-844b-b43f26063235"]
pub struct YamlAsset {
    pub value: String
}

#[derive(Default)]
pub struct YamlAssetLoader;

impl AssetLoader for YamlAssetLoader {
    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut LoadContext,
        ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
            Box::pin(async move {
                let s = std::str::from_utf8(bytes)?;
                let asset = YamlAsset { value: s.into() };
                load_context.set_default_asset(LoadedAsset::new(asset));
                Ok(())
            })
    }

    fn extensions(&self) -> &[&str] {
        &["yaml", "yml"]
    }
}
