use bevy::prelude::*;

use crate::globals::{FONT_SIZE, SIDEBAR_WIDTH};
use crate::input::InputRes;
use crate::manager::{
    CommandEvent, CommandType, GameRes
};
use crate::player::{
    Player,
    upgrades::TransformKind
};
use crate::pieces::components::{
    Health
};

const TILE_BUTTON_DIM: f32 = 64.;
const PAUSE_BUTTON_DIM: f32 = 204.;

#[derive(Component)]
pub struct Sidebar;


#[derive(Component)]
pub struct TileButton{
    pub pressed: bool,
    pub kind: TransformKind
}

#[derive(Component)]
pub struct PauseButton {
    pub pressed: bool
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
                style.size = Size::all(Val::Px(TILE_BUTTON_DIM - 4.));
            },
            Interaction::Hovered | Interaction::None => {
                if button.pressed {
                    input_res.set_mode_by_kind(button.kind, game_res.as_ref());
                    ev_ui.send(super::ReloadUIEvent);
                }
                button.pressed = false;
                style.size = Size::all(Val::Px(TILE_BUTTON_DIM));
            },
            // Interaction::None => {
            //     button.pressed = false;
            //     style.size = Size::all(Val::Px(TILE_BUTTON_DIM));
            // },
        }
    }
}

pub fn pause_button_click(
    mut ev_command: EventWriter<CommandEvent>,
    mut interactions: Query<(&Interaction, &mut PauseButton, &mut Style), Changed<Interaction>>,
) {
    // this should be refactor into some common behaviour with tile buttons
    for (interaction, mut button, mut style) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                button.pressed = true;
                style.size = Size::new(Val::Px(PAUSE_BUTTON_DIM -4.), Val::Px(TILE_BUTTON_DIM - 4.));
            },
            Interaction::Hovered | Interaction::None => {
                if button.pressed {
                    ev_command.send(CommandEvent(CommandType::PlayerWait));
                }
                button.pressed = false;
                style.size = Size::new(Val::Px(PAUSE_BUTTON_DIM), Val::Px(TILE_BUTTON_DIM - 4.));
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
    let container = commands.spawn((
            Sidebar,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { right: Val::Px(0.), ..Default::default() },
                    size: Size::new(Val::Px(SIDEBAR_WIDTH), Val::Percent(100.)),
                    padding: UiRect{ left: Val::Px(4.), ..Default::default()},
                    ..Default::default()
                },
                // background_color: Color::NONE.into(),
                background_color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.3 }.into(),
                ..Default::default()
            }
        ))
        .id(); 
    let content = commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                flex_direction: FlexDirection::Column,
                padding: UiRect{ top: Val::Px(20.), left: Val::Px(24.), ..Default::default()},
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            background_color: super::BG_COLOR.into(),
            // background_color: Color::Rgba { red: 0.18, green: 0.3, blue: 0.42, alpha: 1. }.into(),
            ..Default::default()
        }  
        ))
        .with_children(|parent| {
            spawn_text(parent, assets.as_ref(), "Level ".to_string(), Some((format!("{}", game_res.level), Color::ORANGE_RED)), None);
            spawn_text(parent, assets.as_ref(), "[H] for help".to_string(), None, None);
            spawn_text(parent, assets.as_ref(), String::new(), None, None);
            spawn_text(
                parent,
                assets.as_ref(),
                "AP ".to_string(),
                Some(("O".repeat(game_res.ap as usize), Color::GOLD)),
                Some("O".repeat((game_res.max_ap - game_res.ap) as usize))
            );
            if let Ok(health) = player_query.get_single() {
                spawn_text(
                    parent,
                    assets.as_ref(),
                    "HP ".to_string(),
                    Some(("O".repeat(health.value as usize), Color::ORANGE_RED)),
                    Some("O".repeat((health.max - health.value) as usize))
                );
            };
            // for idx in 1..game_res.tile_transforms.len() + 1 {
            //     let kind = InputRes::get_transform_by_idx(idx);
            //     if !game_res.tile_transforms[&kind] { continue }
            //     spawn_tile_button(
            //         parent,
            //         assets.as_ref(),
            //         input_res.mode == kind,
            //         idx,
            //         kind
            //     );
            // }
            spawn_controls(parent, input_res.as_ref(), game_res.as_ref(), assets.as_ref());
        })
        .id();
    commands.entity(container).add_child(content);
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

fn spawn_controls(
    parent: &mut ChildBuilder,
    input_res: &InputRes,
    game_res: &GameRes,
    assets: &super::UiAssets,
) {
    parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(192.)),
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                // padding: UiRect{ top: Val::Px(20.), left: Val::Px(32.), ..Default::default()},
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
                // pause button
                parent.spawn((
                    PauseButton { pressed: false },
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(PAUSE_BUTTON_DIM), Val::Px(TILE_BUTTON_DIM)),
                            margin: UiRect{ bottom: Val::Px(8.), top: Val::Px(24.), ..Default::default() },
                            ..Default::default()
                        },
                        image: assets.pause_button.clone().into(),
                        ..Default::default()
                    }));

                // tile buttons
                for idx in 1..game_res.tile_transforms.len() + 1 {
                    let kind = InputRes::get_transform_by_idx(idx);
                    if !game_res.tile_transforms[&kind] { continue }
                    spawn_tile_button(
                        parent,
                        assets,
                        input_res.mode == kind,
                        idx,
                        kind
                    );
                }
            });
}

fn spawn_text(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    msg: String,
    color_msg: Option<(String, Color)>,
    dimmed_msg: Option<String>
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
            if let Some(color_msg) = color_msg {
                node.spawn(TextBundle {
                    text: Text::from_section(
                        color_msg.0,
                        TextStyle {
                            color: color_msg.1,
                            font: assets.font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                });
            }
            if let Some(dimmed_msg) = dimmed_msg {
                node.spawn(TextBundle {
                    text: Text::from_section(
                        dimmed_msg,
                        TextStyle {
                            color: Color::BLACK,
                            font: assets.font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                });
            }
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
                size: Size::new(Val::Px(TILE_BUTTON_DIM + 4.), Val::Px(TILE_BUTTON_DIM)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect{ top: Val::Px(20.), ..Default::default() },
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
                        size: Size::all(Val::Px(TILE_BUTTON_DIM)),
                        margin: UiRect::bottom(Val::Px(4.)),
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
