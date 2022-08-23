pub(crate) use crate::player::Player;
pub(crate) use crate::state::GameState;
pub(crate) use crate::utils;
pub use bevy::prelude::*;
pub use iyes_loopless::prelude::*;

pub mod phys {
    pub use crate::utils::phys::group;
    pub use bevy_rapier3d::prelude::*;
}
