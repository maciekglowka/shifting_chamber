use bevy::prelude::*;

use crate::globals::{OVERLAY_FONT_SIZE, SIDEBAR_WIDTH};
use crate::input::InputRes;
use crate::player::Player;
use crate::pieces::components::{
    Health
};

#[derive(Component)]
pub struct Sidebar;

pub fn update_sidebar(
    mut commands: Commands,
    sidebar_query: Query<Entity, With<Sidebar>>,
    player_query: Query<&Health, With<Player>>,
    assets: Res<super::UiAssets>,
    game_res: Res<crate::manager::GameRes>,
    input_res: Res<InputRes>,
    mut ev_ui: EventReader<super::ReloadUIEvent>
) {
    for _ in ev_ui.iter() {
        clear_sidebar(&mut commands, &sidebar_query);
        commands.spawn((
            Sidebar,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { right: Val::Px(0.), ..Default::default() },
                    size: Size::new(Val::Px(SIDEBAR_WIDTH), Val::Percent(100.)),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect{ top: Val::Px(20.), left: Val::Px(20.), ..Default::default()},
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            }  
            ))
            .with_children(|parent| {
                spawn_text(parent, assets.as_ref(), format!("AP: {}", game_res.ap));
                if let Ok(health) = player_query.get_single() {
                    spawn_text(parent, assets.as_ref(), format!("HP: {}/{}", health.value, health.max));
                }
                spawn_text(parent, assets.as_ref(), format!("Mode: {}", game_res.available_transforms[input_res.mode].to_str()));
                spawn_text(parent, assets.as_ref(), "---".to_string());
                spawn_text(parent, assets.as_ref(), "WSAD: move".to_string());
                spawn_text(parent, assets.as_ref(), "Space: change mode".to_string());
                spawn_text(parent, assets.as_ref(), "Enter: wait (save AP)".to_string());
                spawn_text(parent, assets.as_ref(), "I - unit info".to_string());

            });
    }
}

fn clear_sidebar(
    commands: &mut Commands,
    query: &Query<Entity, With<Sidebar>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}

fn spawn_text(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    msg: String
) {
    parent.spawn(TextBundle {
        text: Text::from_section(
            msg,
            TextStyle {
                color: Color::WHITE,
                font: assets.font.clone(),
                font_size: OVERLAY_FONT_SIZE,
                ..Default::default()
            }
        ),
        ..Default::default()
    });
}
