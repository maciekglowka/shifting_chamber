use bevy::prelude::*;

use crate::globals::{OVERLAY_FONT_SIZE, SIDEBAR_WIDTH};
use crate::player::Player;
use crate::pieces::components::Unit;

#[derive(Component)]
pub struct Sidebar;

pub fn update_sidebar(
    mut commands: Commands,
    player_query: Query<(&Player, &Unit)>,
    sidebar_query: Query<Entity, With<Sidebar>>,
    assets: Res<super::UiAssets>
) {
    clear_sidebar(&mut commands, &sidebar_query);

    if let Ok((player, unit)) = player_query.get_single() {
        commands.spawn((
            Sidebar,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { right: Val::Px(0.), ..Default::default() },
                    size: Size::new(Val::Px(SIDEBAR_WIDTH), Val::Percent(100.)),
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            }  
            ))
            .with_children(|parent| {
                spawn_text(parent, assets.as_ref(), 20., 20., format!("HP: {}/{}", unit.hp, unit.max_hp));
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
    top: f32,
    left: f32,
    msg: String
) {
    parent.spawn(TextBundle {
        style: Style {
            position: UiRect { 
                left:  Val::Px(left), 
                top: Val::Px(top),
                ..Default::default()
            },
            ..Default::default()
        },
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