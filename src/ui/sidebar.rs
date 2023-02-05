use bevy::prelude::*;

use crate::actions::{ActionEvent, ActionKind, StatKind};
use crate::globals::{OVERLAY_FONT_SIZE, SIDEBAR_WIDTH};
use crate::manager::{CommandEvent, CommandType};
use crate::player::Player;
use crate::pieces::components::{
    get_effective_dmg,
    Damage,
    Manual,
    Poisoned,
    Protect,
    Temporary,
    Unit
};

#[derive(Component)]
pub struct Sidebar;

pub fn update_sidebar(
    mut commands: Commands,
    player_query: Query<(Entity, &Unit, Option<&Children>, Option<&Poisoned>), With<Player>>,
    damage_query: Query<&Damage>,
    inventory_query: Query<(&Protect, Option<&Temporary>)>,
    manual_item_query: Query<&Manual>,
    sidebar_query: Query<Entity, With<Sidebar>>,
    assets: Res<super::UiAssets>,
    game_res: Res<crate::manager::GameRes>,
    mut ev_ui: EventReader<super::ReloadUIEvent>
) {
    for _ in ev_ui.iter() {
        clear_sidebar(&mut commands, &sidebar_query);

        if let Ok((entity, unit, children, poisoned)) = player_query.get_single() {
            commands.spawn((
                Sidebar,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect { right: Val::Px(0.), ..Default::default() },
                        size: Size::new(Val::Px(SIDEBAR_WIDTH), Val::Percent(100.)),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect{ top: Val::Px(20.), left: Val::Px(20.), ..Default::default()},
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    background_color: Color::NONE.into(),
                    ..Default::default()
                }  
                ))
                .with_children(|parent| {
                    spawn_text(parent, assets.as_ref(), format!("HP: {}/{}", unit.hp(), unit.stats[&StatKind::HP]));
                    if unit.stats.contains_key(&StatKind::ST) {
                        spawn_text(parent, assets.as_ref(), format!("ST: {}", unit.stats[&StatKind::ST]));
                    }
                    spawn_text(parent, assets.as_ref(), format!("Level: {}", game_res.level));
                    spawn_text(parent, assets.as_ref(), format!("Score: {} ({})", game_res.score, game_res.next_upgrade));
                    spawn_text(parent, assets.as_ref(), "-------".into());
                    spawn_text(
                        parent,
                        assets.as_ref(),
                        format!("Eff. dmg: {}", get_effective_dmg(entity, unit, &damage_query, children).1)
                    );
                    if let Some(poisoned) = poisoned {
                        spawn_text(parent, assets.as_ref(), format!("Poisoned: {}", poisoned.value));
                    }
                    spawn_text(parent, assets.as_ref(), "-------".into());

                    for child in children.iter().flat_map(|v| *v) {
                        if let Ok((protect, temp)) = inventory_query.get(*child) {
                            let mut s = format!("Protect: {}", protect.value);
                            if let Some(t) = temp {
                                s += &format!("\nTemporary: {}", t.value);
                            }
                            spawn_text(parent, assets.as_ref(), s);
                        }
                        if let Ok(manual) = manual_item_query.get(*child) {
                            let s = format!("{:?}", manual.kind);
                            add_button(parent, assets.as_ref(), &s, *child);
                        }
                    }
                });
        }
    }
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

#[derive(Component)]
pub struct ItemButton(Entity);

fn add_button(
    parent: &mut ChildBuilder,
    assets: &super::UiAssets,
    msg: &str,
    item: Entity
) {
    parent.spawn((
        ItemButton(item),
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

pub fn button_click(
    mut interactions: Query<(&Interaction, &mut BackgroundColor, &ItemButton), Changed<Interaction>>, 
    mut ev_command: EventWriter<CommandEvent>
) {
    for (interaction, mut color, button) in interactions.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::DARK_GRAY.into();
                ev_command.send(CommandEvent(
                    CommandType::UseItem(button.0))
                );
            },
            _ => {
                *color = Color::BLACK.into()
            },
        }
    }
}