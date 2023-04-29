use bevy::prelude::*;

use crate::globals::{FONT_SIZE, RESTART_PENALTY};
use crate::manager::{GameRes, CommandEvent, CommandType};

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct GameOverButton(bool, CommandType);

const BUTTON_WIDTH: f32 = 480.;
const BUTTON_HEIGHT: f32 = 64.;

pub fn menu_click(
    mut interactions: Query<(&Interaction, &mut GameOverButton, &mut Style), Changed<Interaction>>, 
    mut ev_command: EventWriter<CommandEvent>
) {
    for (interaction, mut button, mut style) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                button.0 = true;
                style.size = Size { width: Val::Px(BUTTON_WIDTH - 8.), height: Val::Px(BUTTON_HEIGHT - 8.)};
            },
            Interaction::Hovered | Interaction::None => {
                if button.0 {
                    ev_command.send(
                        CommandEvent(button.1.clone())
                    );
                }
                button.0 = false;
            }
        }
    }
}

pub fn show_menu(
    mut commands: Commands,
    assets: Res<super::UiAssets>,
    game_res: Res<GameRes>
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
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("Score: {}", game_res.score),
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
            if game_res.score >= RESTART_PENALTY {
                add_button(
                    parent,
                    assets.as_ref(),
                    &format!("Restart Level (cost: {}score)", RESTART_PENALTY),
                    CommandType::RestartLevel
                );
            }
            add_button(parent, assets.as_ref(), "Main Menu", CommandType::RestartGame);
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

fn add_button(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    msg: &str,
    action: CommandType
) {
    parent.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            ..Default::default()
        }
    )
        .with_children(|node| {
            node.spawn((
                GameOverButton(false, action),
                ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                image: assets.button_texture.clone().into(),
                ..Default::default()
            }))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        msg,
                        TextStyle {
                            color: Color::WHITE,
                            font: assets.font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                });
            });
        });
}