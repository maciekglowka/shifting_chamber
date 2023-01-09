use bevy::prelude::*;
use bevy::asset::LoadState;

use crate::states::GameState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_system_set(
                SystemSet::on_update(GameState::LoadAssets)
                    .with_system(check_asset_loading)
            );
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut game_state: ResMut<State<crate::states::GameState>>
) {
    match asset_server.get_group_load_state(
        asset_list.0.iter().map(|a| a.id)
    ) {
        LoadState::Loaded => {
            game_state.set(crate::states::GameState::GameInit)
                .expect("State switch failed!");
        },
        LoadState::Failed => {
            error!("asset loading error");
        },
        _ => {}
    };
}