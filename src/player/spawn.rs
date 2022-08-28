use crate::collider::{ColliderBundle, PhysicsBundle};
use crate::player::PlayerCamera;
use crate::prelude::{phys::*, *};
use crate::viewmodel::{ViewModel, ViewModelBundle, ViewModelHold};
use bevy_rapier3d::geometry::{ActiveCollisionTypes, Collider, Friction};

pub fn build(app: &mut App) {
    app.add_enter_system(GameState::InOffice, Player::spawn);
}

#[derive(Bundle)]
pub struct PlayerBundle {
    this: Player,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    physics: PhysicsBundle,
    dom: Dominance,
}

impl Player {
    pub fn spawn(mut commands: Commands, wrld: &World) {
        commands.spawn_bundle(PlayerBundle {
            this: Player,
            transform: TransformBundle {
                local: Transform::from_translation(Vec3::new(2.0, 2.0, 0.0)),
                global: Default::default(),
            },
            physics: PhysicsBundle {
                body: RigidBody::Dynamic,
                collider: ColliderBundle {
                    collider: Collider::capsule_y(0.9, 0.4),
                    friction: Friction::new(0.7),
                    restitution: Restitution::new(0.3),
                    groups: ActiveCollisionTypes::default(),
                },

                c_groups: group::collide::player_body(),
                locked: LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            },
            dom: Dominance::group(99), // i got 99 problems but getting pushed around by other entities aint one
        });
        commands
            .spawn_bundle(Camera3dBundle {
                camera: Camera {
                    priority: 0,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.4, 0.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            })
            .insert(PlayerCamera);
        let a = commands
            .spawn_bundle(ViewModelBundle {
                transform: TransformBundle::from_transform(
                    Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                        .looking_at(Vec3::default(), Vec3::Y),
                ),
                viewmodel: ViewModel {
                    holding: ViewModelHold::Empty,
                },
            })
            .id();

        let b = commands.spawn().id();
        commands.entity(a).add_child(b);
        commands.entity(a).remove_children(&[b]);
        commands.entity(a).log_components();
    }
}
