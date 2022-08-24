use crate::prelude::*;

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Component)]
pub struct PlayerStateMachine {
    state: PlayerState,
}

impl PlayerStateMachine {
    pub fn state(&self) -> PlayerState {
        self.state
    }

    // Error Handling: lol just dont do bad stuff lol
    pub fn change_state(&mut self, new_state: PlayerState) {
        if self.state.is_change_allowed(new_state) {
            self.state = new_state;
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Component)]
#[allow(dead_code)]
pub enum PlayerState {
    Idle,
    Walking,
    // this is frozen
    Interacting,
    Holding,
}

impl PlayerState {
    pub fn is_change_allowed(&self, new: Self) -> bool {
        match (*self, new) {
            (PlayerState::Idle, _) => true,
            (_, PlayerState::Idle) => true,
            (_, _) => false,
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}
