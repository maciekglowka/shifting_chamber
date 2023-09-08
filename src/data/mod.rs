use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypeUuid, TypePath},
    utils::BoxedFuture
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::states::GameState;

mod levels;
mod pieces;

pub use levels::LevelData;
pub use pieces::{PieceData, SpriteData, SpriteColumns};

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DataAssets>()
            .add_asset::<YamlAsset>()
            .init_asset_loader::<YamlAssetLoader>()
            .add_systems(Startup, pieces::load_assets)
            .add_systems(Startup, levels::load_assets)
            .add_systems(
                OnExit(GameState::LoadAssets),
                (pieces::parse_data, levels::parse_data)
            );
    }
}

#[derive(Default, Resource)]
pub struct DataAssets {
    pub pieces: HashMap<String, PieceData>,
    pub raw_pieces: Vec<(String, Handle<YamlAsset>)>,
    pub unit_names: Vec<String>,
    pub fixture_names: Vec<String>,
    pub raw_levels: Handle<YamlAsset>,
    pub levels: HashMap<String, LevelData>,
    pub level_list: Vec<String>,
    pub raw_level_list: Handle<YamlAsset>
}

#[derive(Debug, Deserialize, TypeUuid, TypePath)]
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
