#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    InOffice,
    GameOver,
}
