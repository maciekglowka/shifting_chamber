use bevy::prelude::*;

use crate::manager::GameRes;

#[derive(Component)]
pub struct GameWinMenu;

pub fn show_menu(
    mut commands: Commands,
    assets: Res<super::UiAssets>,
    game_res: Res<GameRes>
) {
    commands.spawn((
        GameWinMenu,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect { ..Default::default() },
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        }  
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "THE DARK LORD IS DEFEATED!",
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.font.clone(),
                        font_size: 32.,
                        ..Default::default()
                    }
                ),
                ..Default::default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    &format!("Final Score: {}", game_res.score),
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.font.clone(),
                        font_size: 16.,
                        ..Default::default()
                    }
                ),
                style: Style {
                    margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(10.), Val::Px(10.)),
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "(press key or tap to continue)",
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.font.clone(),
                        font_size: 16.,
                        ..Default::default()
                    }
                ),
                ..Default::default()
            });
        });
}

pub fn clear_menu(
    mut commands: Commands,
    query: Query<Entity, With<GameWinMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}