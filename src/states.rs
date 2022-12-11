#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    LoadAssets,
    MapInit,
    PlayerInput,
    TileShift,
    Action
}