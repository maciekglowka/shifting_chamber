use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverMenu;

pub fn show_menu(
    mut commands: Commands,
    assets: Res<super::UiAssets>
) {
    commands.spawn((
        GameOverMenu,
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
                    "GAME OVER",
                    TextStyle {
                        color: Color::WHITE,
                        font: assets.font.clone(),
                        font_size: 32.,
                        ..Default::default()
                    }
                ),
                ..Default::default()
            });
        });
}

pub fn clear_menu(
    mut commands: Commands,
    query: Query<Entity, With<GameOverMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}