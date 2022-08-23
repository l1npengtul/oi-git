use crate::collider::{ColliderBundle, PhysicsBundle};
use crate::prelude::*;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{Collider, Friction, Restitution};

pub fn build(app: &mut App) {
    app.add_enter_system(GameState::MainMenu, Player::spawn);
}

#[derive(Bundle)]
pub struct PlayerBundle {
    this: Player,
    #[bundle]
    camera: Camera3dBundle,
    #[bundle]
    physics: PhysicsBundle,
}

impl Player {
    pub fn spawn(mut commands: Commands) {
        commands.spawn_bundle(PlayerBundle {
            this: Player,
            camera: Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(-1.0, 0.0, 0.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            },
            physics: PhysicsBundle {
                body: RigidBody::KinematicPositionBased,
                collider: ColliderBundle {
                    collider: Collider::cuboid(1.0, 2.0, 1.0),
                    friction: Friction::new(0.7),
                    restitution: Restitution::new(0.3),
                },
            },
        });
    }
}
