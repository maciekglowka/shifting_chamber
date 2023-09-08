use bevy::prelude::*;

use crate::manager::{GameEvent, GameEventKind};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets)
            .add_systems(Update, play_sfx);
    }
}

fn play_sfx(
    mut commands: Commands,
    assets: Res<AudioAssets>,
    mut ev_game: EventReader<GameEvent>
) {
    for ev in ev_game.iter() {
        match ev.0 {
            GameEventKind::ProjectileLaunch => commands.spawn(AudioBundle {
                source: assets.throw_sound.clone(), ..Default::default()
            }),
            GameEventKind::UnitAttack => commands.spawn(AudioBundle {
                source: assets.hit_sound.clone(), ..Default::default()
            }),
            GameEventKind::TileTransformed => commands.spawn(AudioBundle {
                source: assets.tile_sound.clone(), ..Default::default()
            }),
            GameEventKind::WrongAction => commands.spawn(AudioBundle {
                source: assets.wrong_sound.clone(), ..Default::default()
            })
        };
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let hit_sound = asset_server.load("sfx/hit.ogg");
    asset_list.0.push(hit_sound.clone_untyped());

    let wrong_sound = asset_server.load("sfx/no.ogg");
    asset_list.0.push(wrong_sound.clone_untyped());

    let throw_sound = asset_server.load("sfx/throw_2.ogg");
    asset_list.0.push(throw_sound.clone_untyped());

    let tile_sound = asset_server.load("sfx/slide.ogg");
    asset_list.0.push(tile_sound.clone_untyped());

    commands.insert_resource(AudioAssets {
        hit_sound,
        throw_sound,
        tile_sound,
        wrong_sound
    });
}

#[derive(Resource)]
pub struct AudioAssets {
    pub hit_sound: Handle<AudioSource>,
    pub wrong_sound: Handle<AudioSource>,
    pub throw_sound: Handle<AudioSource>,
    pub tile_sound: Handle<AudioSource>,
}