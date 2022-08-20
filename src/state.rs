use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    InGame,
    Paused,
}
