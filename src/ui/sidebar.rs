use bevy::prelude::*;

use crate::actions::StatKind;
use crate::globals::{OVERLAY_FONT_SIZE, SIDEBAR_WIDTH};
use crate::player::Player;
use crate::pieces::components::{
    Protect,
    Temporary,
    Unit
};

#[derive(Component)]
pub struct Sidebar;

pub fn update_sidebar(
    mut commands: Commands,
    player_query: Query<(&Player, &Unit, Option<&Children>)>,
    inventory_query: Query<(&Protect, Option<&Temporary>)>,
    sidebar_query: Query<Entity, With<Sidebar>>,
    assets: Res<super::UiAssets>,
    game_res: Res<crate::manager::GameRes>
) {
    clear_sidebar(&mut commands, &sidebar_query);

    if let Ok((_player, unit, children)) = player_query.get_single() {
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
                spawn_text(parent, assets.as_ref(), format!("Score: {}", game_res.score));
                spawn_text(parent, assets.as_ref(), "-------".into());

                for child in children.iter().flat_map(|v| *v) {
                    if let Ok((protect, temp)) = inventory_query.get(*child) {
                        let mut s = format!("Protect: {}", protect.value);
                        if let Some(t) = temp {
                            s += &format!("\nTemporary: {}", t.value);
                        }
                        spawn_text(parent, assets.as_ref(), s);
                    }
                }
            });
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