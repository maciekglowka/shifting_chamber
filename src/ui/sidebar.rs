use bevy::prelude::*;

use crate::globals::{FONT_SIZE, SIDEBAR_WIDTH};
use crate::input::InputRes;
use crate::manager::GameRes;
use crate::player::{
    Player,
    upgrades::TransformKind
};
use crate::pieces::components::{
    Health
};

const TILE_BUTTION_DIM: f32 = 64.;

#[derive(Component)]
pub struct Sidebar;


#[derive(Component)]
pub struct TileButton{
    pub pressed: bool,
    pub kind: TransformKind
}

pub fn tile_button_click(
    mut interactions: Query<(&Interaction, &mut TileButton, &mut Style), Changed<Interaction>>,
    mut input_res: ResMut<InputRes>,
    game_res: Res<GameRes>,
    mut ev_ui: EventWriter<super::ReloadUIEvent>
) {
    for (interaction, mut button, mut style) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                button.pressed = true;
                style.size = Size::all(Val::Px(TILE_BUTTION_DIM - 4.));
            },
            Interaction::Hovered => {
                if button.pressed {
                    input_res.set_mode_by_kind(button.kind, game_res.as_ref());
                    ev_ui.send(super::ReloadUIEvent);
                }
                button.pressed = false;
                style.size = Size::all(Val::Px(TILE_BUTTION_DIM));
            },
            Interaction::None => {
                button.pressed = false;
                style.size = Size::all(Val::Px(TILE_BUTTION_DIM));
            },
        }
    }
}

pub fn update_sidebar(
    mut commands: Commands,
    sidebar_query: Query<Entity, With<Sidebar>>,
    player_query: Query<&Health, With<Player>>,
    assets: Res<super::UiAssets>,
    game_res: Res<crate::manager::GameRes>,
    input_res: Res<InputRes>,
    mut ev_ui: EventReader<super::ReloadUIEvent>
) {
    if ev_ui.iter().len() == 0 { return };
    clear_sidebar(&mut commands, &sidebar_query);
    commands.spawn((
        Sidebar,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect { right: Val::Px(0.), ..Default::default() },
                size: Size::new(Val::Px(SIDEBAR_WIDTH), Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                padding: UiRect{ top: Val::Px(80.), left: Val::Px(32.), ..Default::default()},
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            // background_color: Color::NONE.into(),
            background_color: Color::Rgba { red: 0.12, green: 0.2, blue: 0.28, alpha: 1. }.into(),
            ..Default::default()
        }  
        ))
        .with_children(|parent| {
            for idx in 1..game_res.tile_transforms.len() + 1 {
                let kind = InputRes::get_transform_by_idx(idx);
                if !game_res.tile_transforms[&kind] { continue }
                spawn_tile_button(
                    parent,
                    assets.as_ref(),
                    input_res.mode == kind,
                    idx,
                    kind
                );
            }
            spawn_text(
                parent,
                assets.as_ref(),
                format!("AP {}", "O".repeat(game_res.ap as usize)),
                "O".repeat((game_res.max_ap - game_res.ap) as usize)
            );
            if let Ok(health) = player_query.get_single() {
                spawn_text(
                    parent,
                    assets.as_ref(),
                    format!("HP {}", "O".repeat(health.value as usize)),
                    "O".repeat((health.max - health.value) as usize)    
                );
            };
            spawn_text(parent, assets.as_ref(), "[H] for help".to_string(), String::new());
        });
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
    msg: String,
    dimmed_msg: String
) {
    parent.spawn(NodeBundle {
        style: Style {
            margin: UiRect { top: Val::Px(4.), bottom: Val::Px(4.), ..Default::default() },
            ..Default::default()
        },
        ..Default::default()
    })
        .with_children(|node| {
            node.spawn(TextBundle {
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
            node.spawn(TextBundle {
                text: Text::from_section(
                    dimmed_msg,
                    TextStyle {
                        color: Color::GRAY,
                        font: assets.font.clone(),
                        font_size: FONT_SIZE,
                        ..Default::default()
                    }
                ),
                ..Default::default()
            });
        });
}

fn spawn_tile_button(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    active: bool,
    idx: usize,
    kind: TransformKind
) {
    parent.spawn(
        NodeBundle {
            style: Style {
                size: Size { height: Val::Px(TILE_BUTTION_DIM), ..Default::default() },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                margin: UiRect{ bottom: Val::Px(20.), ..Default::default() },
                ..Default::default()
            },
            ..Default::default()
        }
    )
        .with_children(|node| {
            node.spawn((
                TileButton{ pressed: false, kind },
                ButtonBundle {
                    style: Style {
                        size: Size::all(Val::Px(TILE_BUTTION_DIM)),
                        margin: UiRect::right(Val::Px(8.)),
                        ..Default::default()
                    },
                    image: assets.tile_buttons[&kind].clone().into(),
                    ..Default::default()
                }));
                node.spawn(TextBundle {
                    text: Text::from_section(
                        format!{"[{}]", idx},
                        TextStyle {
                            color: if active { Color::WHITE } else { Color::GRAY },
                            font: assets.font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                });
            });
}
