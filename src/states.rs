use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    LoadAssets,
    GameInit,
    MapInit,
    TurnStart,
    PlayerInput,
    TileShift,
    NPCAction,
    NPCResult,
    TurnEnd,
    MapEnd,
    GameOver,
}