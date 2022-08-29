use crate::prelude::*;

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct PlayerStateMachine {
    state: PlayerState,
}

impl PlayerStateMachine {
    // pub const IDLE: PlayerStateMachine = PlayerStateMachine {
    //     state: PlayerState::Idle,
    // };
    // pub const WALKING: PlayerStateMachine = PlayerStateMachine {
    //     state: PlayerState::Walking,
    // };
    pub const INTERACTING: PlayerStateMachine = PlayerStateMachine {
        state: PlayerState::Interacting,
    };
    // pub const HOLDING: PlayerStateMachine = PlayerStateMachine {
    //     state: PlayerState::Holding,
    // };

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
