use crate::collider::{ColliderBundle, PhysicsBundle, CG_DYNAMIC, CG_PLAYER, CG_STATIC};
use crate::player::{PlayerCamera, PlayerPhysics};
use crate::prelude::*;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{
    ActiveCollisionTypes, Collider, CollisionGroups, Friction, Restitution,
};
use bevy_rapier3d::prelude::{AdditionalMassProperties, LockedAxes};

pub fn build(app: &mut App) {
    app.add_enter_system(GameState::MainMenu, Player::spawn);
}

#[derive(Bundle)]
pub struct PlayerBundle {
    this: Player,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    physics: PhysicsBundle,
}

impl Player {
    pub fn spawn(mut commands: Commands) {
        commands
            .spawn_bundle(PlayerBundle {
                this: Player,
                transform: TransformBundle {
                    local: Transform::from_translation(Vec3::new(2.0, 2.2, 0.0))
                        .looking_at(Vec3::default(), Vec3::Y),
                    global: Default::default(),
                },
                physics: PhysicsBundle {
                    body: RigidBody::KinematicPositionBased,
                    collider: ColliderBundle {
                        collider: Collider::cuboid(0.4, 1.0, 0.4),
                        friction: Friction::new(0.7),
                        restitution: Restitution::new(0.3),
                        groups: ActiveCollisionTypes::all(),
                    },
                    c_groups: CollisionGroups::new(CG_PLAYER, CG_DYNAMIC | CG_STATIC),
                    mass: AdditionalMassProperties::Mass(10_f32), // TODO: Adjust
                    locked: LockedAxes::empty(),
                },
            })
            .with_children(|b| {
                b.spawn_bundle(Camera3dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                        .looking_at(Vec3::default(), Vec3::Y),
                    ..Default::default()
                })
                .insert(PlayerCamera);
            });
    }
}
