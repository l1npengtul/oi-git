use bevy_rapier3d::prelude::{ActiveCollisionTypes, Collider, LockedAxes, RigidBody, AdditionalMassProperties, CollisionGroups, Friction, Restitution};
use crate::{Bundle, Component};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Component)]
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
pub const CG_STATIC: u32 = 0b1;
pub const CG_DYNAMIC: u32 = 0b10;
pub const CG_PLAYER: u32 = 0b100;

pub const WEIGHT_LOC: f32 = 10_f32; // TODO: Adjust
pub const WEIGHT_TOOL: f32 = 20_f32;

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub body: RigidBody,
    #[bundle]
    pub collider: ColliderBundle,
    pub c_groups: CollisionGroups,
    pub mass: AdditionalMassProperties,
    pub locked: LockedAxes,
}

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub restitution: Restitution,
    pub groups: ActiveCollisionTypes,
}
