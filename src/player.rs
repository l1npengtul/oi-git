use crate::prelude::*;

pub mod look;
pub mod movement;
pub mod spawn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        spawn::build(app);
        movement::build(app);
        look::build(app);
    }
}

#[derive(Component)]
pub struct Player;
