use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
}

pub struct StatePlugin {
    pub initial: GameState,
}
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(self.initial.clone());
    }
}
