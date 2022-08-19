use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    MainMenu,
    InGame,
    Paused,
}

pub const INITIAL: State = State::MainMenu;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(INITIAL);
    }
}
