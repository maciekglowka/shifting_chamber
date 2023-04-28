use bevy::prelude::*;
use rand::prelude::*;

use crate::globals::{FONT_SIZE, UPGRADE_CHOICES, UPGRADE_PENALTY};
use crate::manager::{CommandEvent, CommandType};
use crate::player::upgrades::UpgradeKind;

#[derive(Component)]
pub struct UpgradeMenu;

#[derive(Component)]
pub struct UpgradeButton(bool, UpgradeKind);

const BUTTON_WIDTH: f32 = 480.;
const BUTTON_HEIGHT: f32 = 64.;

pub fn menu_click(
    mut interactions: Query<(&Interaction, &mut UpgradeButton, &mut Style), Changed<Interaction>>, 
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
                        CommandEvent(CommandType::Upgrade(button.1.clone()))
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
    game_res: Res<crate::manager::GameRes>
) {
    commands.spawn((
            UpgradeMenu,
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                ..Default::default()
            }
        ))
        .with_children(|parent| {
            parent.spawn(
                    NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            // padding: UiRect::all(Val::Px(20.)),
                            flex_direction: FlexDirection::Column,
                            size: Size { width: Val::Percent(100.), height: Val::Percent(100.) },
                            ..Default::default()
                        },
                        background_color: super::BG_COLOR.into(),
                        ..Default::default()
                    }
                )
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("Choose your upgrade (-{} score):", UPGRADE_PENALTY),
                            TextStyle {
                                color: Color::WHITE,
                                font: assets.font.clone(),
                                font_size: FONT_SIZE,
                                ..Default::default()
                            }
                        ),
                        style: Style {
                            margin: UiRect::bottom(Val::Px(32.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    let mut rng = thread_rng();
                    for choice in game_res.possible_upgrades.iter().choose_multiple(&mut rng, UPGRADE_CHOICES) {
                        add_button(parent, assets.as_ref(), choice.to_str(), *choice);
                    }
                    add_button(parent, assets.as_ref(), UpgradeKind::Skip.to_str(), UpgradeKind::Skip);
                });
        });
}

pub fn clear_menu(
    mut commands: Commands,
    query: Query<Entity, With<UpgradeMenu>>
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
    action: UpgradeKind
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
                UpgradeButton(false, action),
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