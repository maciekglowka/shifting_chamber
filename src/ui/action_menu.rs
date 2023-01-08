use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind};
use crate::globals::OVERLAY_FONT_SIZE;
use crate::manager::GameRes;

#[derive(Component)]
pub struct ActionMenu;

#[derive(Component)]
pub struct ActionButton(ActionKind);

pub fn menu_click(
    mut commands: Commands,
    mut interactions: Query<(Entity, &Interaction, &mut BackgroundColor, &ActionButton), Changed<Interaction>>, 
    mut ev_action: EventWriter<ActionEvent>
) {
    for (entity, interaction, mut color, button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::DARK_GRAY.into();
                ev_action.send(ActionEvent(button.0));
                // TODO remove action from game res as well?
                commands.entity(entity)
                    .despawn_recursive()
            },
            _ => {
                *color = Color::BLACK.into()
            },
        }
    }
}

pub fn update_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<ActionMenu>>,
    assets: Res<super::UiAssets>,
    game_res: Res<GameRes>
) {
    clear_menu(&mut commands, &menu_query);

    commands.spawn((
        ActionMenu,
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
            for action in game_res.input_actions.iter() {
                add_button(
                    &mut parent,
                    &assets,
                    &format!("{:?}", action),
                    *action
                );
            }
        });
}

fn clear_menu(
    commands: &mut Commands,
    query: &Query<Entity, With<ActionMenu>>
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
    action: ActionKind
) {
    parent.spawn((
        ActionButton(action),
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