use bevy::prelude::*;

use crate::globals::{OVERLAY_FONT_SIZE, SIDEBAR_WIDTH};
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
    pub available: bool,
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
                if button.available {
                    button.pressed = true;
                    style.size = Size::all(Val::Px(TILE_BUTTION_DIM - 4.));
                }
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
                padding: UiRect{ top: Val::Px(64.), left: Val::Px(10.), ..Default::default()},
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        }  
        ))
        .with_children(|parent| {
            spawn_text(parent, assets.as_ref(), format!("AP: {}/{}", game_res.ap, game_res.max_ap));
            if let Ok(health) = player_query.get_single() {
                spawn_text(parent, assets.as_ref(), format!("HP: {}/{}", health.value, health.max));
            }
            for (kind, available) in game_res.tile_transforms.iter() {
                spawn_tile_button(
                    parent,
                    assets.as_ref(),
                    input_res.mode == *kind,
                    *available,
                    0,
                    *kind
                );
            }
            spawn_text(parent, assets.as_ref(), "---".to_string());
            spawn_text(parent, assets.as_ref(), "WASD: move".to_string());
            spawn_text(parent, assets.as_ref(), "[AD when rotating]".to_string());
            spawn_text(parent, assets.as_ref(), "Space: wait (stack AP)".to_string());
            spawn_text(parent, assets.as_ref(), "Enter: change mode".to_string());
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
    msg: String
) {
    parent.spawn(TextBundle {
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
}

fn spawn_tile_button(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    active: bool,
    available: bool,
    idx: usize,
    kind: TransformKind
) {
    parent.spawn(
        NodeBundle {
            style: Style {
                size: Size::all(Val::Px(TILE_BUTTION_DIM)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                margin: UiRect::all(Val::Px(8.)),
                ..Default::default()
            },
            ..Default::default()
        }
    )
        .with_children(|node| {
            node.spawn((
                TileButton{ available, pressed: false, kind },
                ButtonBundle {
                    style: Style {
                        size: Size::all(Val::Percent(100.)),
                        margin: UiRect::right(Val::Px(8.)),
                        ..Default::default()
                    },
                    image: assets.tile_buttons[&kind].clone().into(),
                    ..Default::default()
                }));
                node.spawn(TextBundle {
                    text: Text::from_section(
                        format!{"[{}]", idx + 1},
                        TextStyle {
                            color: if active { Color::WHITE } else { Color::GRAY },
                            font: assets.font.clone(),
                            font_size: OVERLAY_FONT_SIZE,
                            ..Default::default()
                        }
                    ),
                    ..Default::default()
                });
            });
}

