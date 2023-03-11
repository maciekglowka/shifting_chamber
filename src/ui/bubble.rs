use bevy::prelude::*;
use rand::prelude::*;

use crate::globals::{BUBBLE_LIFE, OVERLAY_FONT_SIZE, BUBBLE_Z, TILE_SIZE};
use crate::graphics::get_world_position;
use crate::vectors::Vector2Int;

const BUBBLE_SPEED: f32 = 30.;

#[derive(Component)]
pub struct Bubble {
    pub age: f32
}

pub struct BubbleEvent(pub Vector2Int, pub String);

pub fn update_bubbles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Bubble)>
) {
    for (entity, mut transform, mut bubble) in query.iter_mut() {
        bubble.age += time.delta_seconds();
        if bubble.age > BUBBLE_LIFE {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let v = time.delta_seconds() * Vec3::new(0., BUBBLE_SPEED, 0.);
        transform.translation += v;
    }
}

pub fn spawn_bubbles(
    mut ev_bubble: EventReader<BubbleEvent>,
    mut commands: Commands,
    res: Res<super::UiAssets>
) {
    for ev in ev_bubble.iter() {
        let style = TextStyle {
            font: res.font.clone(),
            font_size: OVERLAY_FONT_SIZE,
            color: Color::MAROON
        };
        let mut rng = thread_rng();
        let offset = TILE_SIZE / 16.;
        let offset_v = Vec3::new(rng.gen_range(-offset..offset), rng.gen_range(-offset..offset), 0.);
        let v = get_world_position(ev.0, BUBBLE_Z) + offset_v;
        commands.spawn((
            Bubble { age: 0. },
            Text2dBundle {
                text: Text::from_section(ev.1.clone(), style),
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
    }
}