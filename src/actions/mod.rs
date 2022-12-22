use bevy::prelude::*;

mod player;

pub struct ActionEvent(pub ActionKind);

#[derive(Clone, Copy, Debug)]
pub enum ActionKind {
    Descend,
    Heal(u32)
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_system(player::descend)
            .add_system(player::heal);
    }
}
