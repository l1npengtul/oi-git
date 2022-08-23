use crate::prelude::{*, phys::*};

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub body: RigidBody,
    #[bundle]
    pub collider: ColliderBundle,
    pub c_groups: CollisionGroups,
    pub mass: AdditionalMassProperties,
    pub locked: LockedAxes,
    pub vel: Velocity, 
}

#[derive(Bundle, Default)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub friction: Friction,
    pub restitution: Restitution,
    pub groups: ActiveCollisionTypes,
}
