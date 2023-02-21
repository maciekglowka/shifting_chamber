#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    LoadAssets,
    GameInit,
    MapInit,
    TurnStart,
    PlayerInput,
    TileShift,
    NPCMove,
    MoveResult,
    TurnEnd,
    Upgrade,
    GameOver,
}