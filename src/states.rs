#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    LoadAssets,
    GameInit,
    MapInit,
    PlayerInput,
    TileShift,
    ShiftResult,
    Upgrade,
    GameOver,
}