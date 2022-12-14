use crate::player::fsm::PlayerStateMachine;
use crate::prelude::*;
pub use interactions::{MouseInteraction, PlayerLookingAt};

pub mod fsm;
pub mod interactions;
pub mod look;
pub mod movement;
pub mod spawn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        spawn::build(app);
        movement::build(app);
        look::build(app);
        interactions::build(app);
        app.insert_resource(PlayerStateMachine::default());
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerPhysics;
