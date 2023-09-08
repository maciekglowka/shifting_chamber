use bevy::prelude::*;
use bevy::asset::LoadState;

use crate::states::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_systems(Update, check_asset_loading.run_if(in_state(GameState::LoadAssets)));
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<crate::states::GameState>>
) {
    match asset_server.get_group_load_state(
        asset_list.0.iter().map(|a| a.id())
    ) {
        LoadState::Loaded => {
            next_state.set(crate::states::GameState::MainMenu);
        },
        LoadState::Failed => {
            error!("asset loading error");
        },
        _ => {}
    };
}