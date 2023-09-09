use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::actions::{Action, ActionEvent};
use crate::globals::{BUTTON_CLICK_OFFSET, FONT_SIZE};

const BUTTON_WIDTH: f32 = 480.;
const BUTTON_HEIGHT: f32 = 64.;

#[derive(Event)]
pub struct CloseModalEvent;

#[derive(Component)]
pub struct Modal;

#[derive(Component)]
pub struct ModalButton {
    pub clicked: bool,
    pub action: Option<Box<dyn Action>>
}

pub fn spawn_modal(
    commands: &mut Commands,
    text: String,
    options: Vec<(String, Box<dyn Action>)>,
    assets: &super::UiAssets,
) {
    let container = commands.spawn((
            Modal,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
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
                style: Style {
                    margin: UiRect::bottom(Val::Px(32.)),
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: text,
                            style: TextStyle {
                                color: Color::WHITE,
                                font: assets.font.clone(),
                                font_size: 32.,
                                ..Default::default()
                            }
                        }
                    ],
                    linebreak_behavior: BreakLineOn::WordBoundary,
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    spawn_buttons(commands, container, options, assets);
}

fn spawn_buttons(
    commands: &mut Commands,
    parent: Entity,
    options: Vec<(String, Box<dyn Action>)>,
    assets: &super::UiAssets,
) {
    for option in options {
        let button_container = commands.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Px(BUTTON_WIDTH),
                        height: Val::Px(BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            )
            .id();
        commands.entity(parent).add_child(button_container);

        let button = commands.spawn((
                ModalButton { clicked: false, action: Some(option.1) },
                ButtonBundle {
                    style: Style {
                        width: Val::Px(BUTTON_WIDTH),
                        height: Val::Px(BUTTON_HEIGHT),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    image: assets.button_texture.clone().into(),
                    ..Default::default()
                }
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        option.0,
                        TextStyle {
                            color: Color::WHITE,
                            font: assets.font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                });
            })
            .id();
        commands.entity(button_container).add_child(button);
    }
}

pub fn button_click(
    mut interactions: Query<(&Interaction, &mut ModalButton, &mut Style), Changed<Interaction>>, 
    mut ev_action: EventWriter<ActionEvent>,
    mut ev_modal: EventWriter<CloseModalEvent>
) {
    for (interaction, mut button, mut style) in interactions.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                button.clicked = true;
                style.width = Val::Px(BUTTON_WIDTH - BUTTON_CLICK_OFFSET);
                style.height = Val::Px(BUTTON_HEIGHT - BUTTON_CLICK_OFFSET);
            }
            Interaction::Hovered => {
                if button.clicked {
                    if let Some(action) = button.action.take() {
                        ev_action.send(ActionEvent(action));
                    }
                    ev_modal.send(CloseModalEvent);
                }
                button.clicked = false;
            },
            Interaction::None => {
                if button.clicked {
                    button.clicked = false;
                    style.width = Val::Px(BUTTON_WIDTH);
                    style.height = Val::Px(BUTTON_HEIGHT);
                }
            }
        }
    }
}

pub fn clear_modal(
    mut commands: Commands,
    query: Query<Entity, With<Modal>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}