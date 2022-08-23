use crate::prelude::*;

pub mod interactions;
pub use interactions::MouseInteraction;
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
    }
}

#[derive(Component)]
pub struct Player;
