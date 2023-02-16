use bevy::prelude::*;

// use crate::actions::{ActionKind, StatKind};
use crate::globals::OVERLAY_FONT_SIZE;
use crate::manager::{CommandEvent, CommandType};

#[derive(Component)]
pub struct UpgradeMenu;

// #[derive(Component)]
// pub struct UpgradeButton(ActionKind);

// pub fn menu_click(
//     mut interactions: Query<(&Interaction, &mut BackgroundColor, &UpgradeButton), Changed<Interaction>>, 
//     mut ev_command: EventWriter<CommandEvent>
// ) {
//     for (interaction, mut color, button) in interactions.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 *color = Color::DARK_GRAY.into();
//                 ev_command.send(
//                     CommandEvent(CommandType::Upgrade(button.0.clone()))
//                 );
//             },
//             _ => {
//                 *color = Color::BLACK.into()
//             },
//         }
//     }
// }

// pub fn show_menu(
//     mut commands: Commands,
//     assets: Res<super::UiAssets>,
// ) {
//     commands.spawn((
//             UpgradeMenu,
//             NodeBundle {
//                 style: Style {
//                     align_items: AlignItems::Center,
//                     justify_content: JustifyContent::Center,
//                     size: Size::new(Val::Percent(100.), Val::Percent(100.)),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             }
//         ))
//         .with_children(|parent| {
//             parent.spawn(
//                     NodeBundle {
//                         style: Style {
//                             align_items: AlignItems::Center,
//                             justify_content: JustifyContent::Center,
//                             padding: UiRect::all(Val::Px(20.)),
//                             flex_direction: FlexDirection::Column,
//                             ..Default::default()
//                         },
//                         background_color: Color::DARK_GRAY.into(),
//                         ..Default::default()
//                     }
//                 )
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle {
//                         text: Text::from_section(
//                             "Choose your upgrade:",
//                             TextStyle {
//                                 color: Color::WHITE,
//                                 font: assets.font.clone(),
//                                 font_size: OVERLAY_FONT_SIZE,
//                                 ..Default::default()
//                             }
//                         ),
//                         ..Default::default()
//                     });
//                     add_button(parent, assets.as_ref(), "HP +3", ActionKind::Heal(3));
//                     add_button(parent, assets.as_ref(), "MAX HP + 1", ActionKind::StatUpgrade(StatKind::HP, 1));
//                     add_button(parent, assets.as_ref(), "ST + 1", ActionKind::StatUpgrade(StatKind::ST, 1));
//                 });
//         });
// }

// pub fn clear_menu(
//     mut commands: Commands,
//     query: Query<Entity, With<UpgradeMenu>>
// ) {
//     for entity in query.iter() {
//         commands.entity(entity)
//             .despawn_recursive();
//     }
// }

// fn add_button(
//     parent: &mut ChildBuilder,
//     assets: &super::UiAssets,
//     msg: &str,
//     action: ActionKind
// ) {
//     parent.spawn((
//         UpgradeButton(action),
//         ButtonBundle {
//             style: Style {
//                 size: Size::new(Val::Px(200.), Val::Px(32.)),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 margin: UiRect::all(Val::Px(10.)),
//                 ..Default::default()
//             },
//             background_color: Color::BLACK.into(),
//             ..Default::default()
//         }
//     ))
//         .with_children(|button| {
//             button.spawn(TextBundle {
//                 text: Text::from_section(
//                     msg,
//                     TextStyle {
//                         color: Color::WHITE,
//                         font: assets.font.clone(),
//                         font_size: OVERLAY_FONT_SIZE,
//                         ..Default::default()
//                     }
//                 ),
//                 ..Default::default()
//             });
//         });
// }