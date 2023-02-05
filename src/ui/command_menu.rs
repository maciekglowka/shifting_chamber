use bevy::prelude::*;

use crate::actions::{ActionKind, ActionEvent};
use crate::manager::{CommandEvent, CommandType, GameRes};
use crate::globals::OVERLAY_FONT_SIZE;

#[derive(Component)]
pub struct CommandMenu;

#[derive(Component)]
pub struct CommandButton(CommandType);

pub fn menu_click(
    mut interactions: Query<(&Interaction, &mut BackgroundColor, &CommandButton), Changed<Interaction>>, 
    mut ev_command: EventWriter<CommandEvent>
) {
    for (interaction, mut color, button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::DARK_GRAY.into();
                ev_command.send(CommandEvent(button.0.clone()));
            },
            _ => {
                *color = Color::BLACK.into()
            },
        }
    }
}

pub fn update_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<CommandMenu>>,
    assets: Res<super::UiAssets>,
    game_res: Res<GameRes>,
    mut ev_ui: EventReader<super::ReloadUIEvent>
) {
    for _ in ev_ui.iter() {
        clear_menu(&mut commands, &menu_query);
        commands.spawn((
                CommandMenu,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        padding: UiRect { left: Val::Px(20.), top: Val::Px(20.), ..Default::default()},
                        // size: Size::new(Val::Px(SIDEBAR_WIDTH), Val::Percent(100.)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ))
            .with_children(|mut parent| {
                for command in game_res.input_commands.iter() {
                    add_button(
                        &mut parent,
                        &assets,
                        &format!("{:?}", command),
                        command.clone()
                    );
                }
            });
    }
}

fn clear_menu(
    commands: &mut Commands,
    query: &Query<Entity, With<CommandMenu>>
) {
    for entity in query.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}

fn add_button(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    msg: &str,
    command: CommandType
) {
    parent.spawn((
        CommandButton(command),
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.), Val::Px(32.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        }
    ))
        .with_children(|button| {
            button.spawn(TextBundle {
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
        });
}