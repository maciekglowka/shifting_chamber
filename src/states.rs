use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    LoadAssets,
    MainMenu,
    GameInit,
    MapInit,
    TurnStart,
    PlayerInput,
    TileShift,
    NPCAction,
    NPCResult,
    TurnEnd,
    MapEnd,
    Upgrade,
    GameOver,
    GameWin
}