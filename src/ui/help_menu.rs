use bevy::prelude::*;

use crate::input::InputRes;

#[derive(Component)]
pub struct HelpMenu;

pub fn toggle_menu(
    mut commands: Commands,
    assets: Res<super::UiAssets>,
    input_res: Res<InputRes>,
    query: Query<Entity, With<HelpMenu>>,
    mut ev_ui: EventReader<super::ReloadUIEvent>
) {
    if ev_ui.iter().len() == 0 { return };
    destroy_menu(&mut commands, &query);
    if input_res.show_help {
        show_menu(&mut commands, assets);
    }
}

pub fn show_menu(
    commands: &mut Commands,
    assets: Res<super::UiAssets>,
) {
    commands.spawn((
        HelpMenu,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect { left: Val::Px(100.), ..Default::default() },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: super::BG_COLOR.into(),
            ..Default::default()
        }  
        ))
        .with_children(|mut parent| {
            spawn_text(&mut parent, &assets, "Use WASD keys to transform the map. (AD when rotating)");
            spawn_text(&mut parent, &assets, "At first you have only one transform mode available - Tile Shift");
            spawn_text(&mut parent, &assets, "You can pause by pressing space bar, and stack AP");
            spawn_text(&mut parent, &assets, "(if max AP is greater than 1)");
            spawn_text(&mut parent, &assets, "You cannot directly attack your opponents");
            spawn_text(&mut parent, &assets, "Manipulate the board to destroy them!");
            spawn_text(&mut parent, &assets, "There are 10 levels ahead");
        });
}

pub fn clear_menu(
    mut commands: Commands,
    query: Query<Entity, With<HelpMenu>>
) {
    destroy_menu(&mut commands, &query);
}

fn destroy_menu(
    commands: &mut Commands,
    query: &Query<Entity, With<HelpMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}

fn spawn_text(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    msg: &str
) {
    parent.spawn(
        NodeBundle {
            style : Style {
                margin: UiRect { bottom: Val::Px(8.), ..Default::default() },
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        msg,
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