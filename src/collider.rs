use crate::Bundle;
use bevy_rapier3d::prelude::{ActiveCollisionTypes, Collider, RigidBody};

pub const WALL_COL_TYPES: ActiveCollisionTypes =
    ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC;
pub const OBJECT_COL_TYPES: ActiveCollisionTypes =
    ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_KINEMATIC;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CollisionGroup(u32, u32);

impl CollisionGroup {
    pub fn memberships(&self) -> u32 {
        self.0
    }

    pub fn filters(&self) -> u32 {
        self.1
    }
}

impl Default for CollisionGroup {
    fn default() -> Self {
        CollisionGroup(u32::MAX, 0)
    }
}

pub type SolverGroup = CollisionGroup;

pub const CG_ENV: CollisionGroup = CollisionGroup(0b1, u32::MAX);
pub const CG_OBJECTS: CollisionGroup = CollisionGroup(0b10, u32::MAX);
pub const CG_PLAYER: CollisionGroup = CollisionGroup(0b100, u32::MAX);

pub const SG_OBJECTS: SolverGroup = CollisionGroup(0b10, 0b100);

pub const WEIGHT_LOC: f32 = 10_f32; // TODO: Adjust
pub const WEIGHT_TOOL: f32 = 20_f32;

#[derive(Bundle)]
pub struct KinematicBodyBundle {
    body: RigidBody,
    collider: Collider,
}
