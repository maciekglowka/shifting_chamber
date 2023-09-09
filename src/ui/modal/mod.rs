use bevy::prelude::*;
use bevy::ecs::system::Command;

#[derive(Resource)]
pub struct ModalState {
    pub modal: Option<Modal>
}

pub struct Modal {
    pub text: String,
    pub buttons: Vec<Entity>
}

// #[derive(Component)]
// pub struct ModalButton<T> {
//     pub command: Box<dyn Command>
// }