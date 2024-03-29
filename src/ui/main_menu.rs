use bevy::prelude::*;


#[derive(Component)]
pub struct MainMenu;


pub fn show_menu(
    mut commands: Commands,
    assets: Res<super::UiAssets>,
) {
    commands.spawn((
        MainMenu,
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
            z_index: ZIndex::Global(200),
            background_color: super::BG_COLOR.into(),
            ..Default::default()
        }  
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: assets.title_screen.clone().into(),
                ..Default::default()
            });
        });
}

pub fn clear_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}
