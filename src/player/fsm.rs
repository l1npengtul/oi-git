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
    pub fn change_state(&mut self, new_state: PlayerStateMachine) {
        if self.state.is_change_allowed(new_state.state) {
            self.state = new_state.state;
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Component)]
pub enum PlayerState {
    Idle,
    Walking,
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
