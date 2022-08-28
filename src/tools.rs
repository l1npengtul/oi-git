use crate::collider::{ColliderBundle, PhysicsBundle};
use crate::interactable::Interactable;
use crate::level::NewLevel;
use crate::office::SceneLocations;
use crate::phys::group::collide::{dynamic_body, interactable_body, interactable_dynamic_body};
use crate::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_rapier3d::prelude::{Collider, RigidBody};

#[derive(AssetCollection)]
pub struct HammerModel {
    #[asset(path = "tools_and_viewmodels/hammer.glb#Scene0")]
    pub hammer_scene: Handle<Scene>,
    #[asset(path = "tools_and_viewmodels/hammer.glb#Animation0")]
    pub swing_animation: Handle<AnimationClip>,
}

pub struct ToolsPlugin;

impl ToolsPlugin {
    pub fn spawn_hammer(
        mut commands: Commands,
        locations: Res<SceneLocations>,
        hammer: Res<HammerModel>,
    ) {
        let mut hammer_spawn = *locations.locations.get("point3d_tooldesk").unwrap();
        hammer_spawn.scale = Vec3::ONE;
        hammer_spawn.rotate_local_y(1.57);

        println!("{}", hammer_spawn.translation);
        commands
            .spawn()
            .insert_bundle(SceneBundle {
                scene: hammer.hammer_scene.clone(),
                transform: hammer_spawn,
                ..Default::default()
            })
            .insert_bundle(PhysicsBundle {
                body: RigidBody::Dynamic,
                collider: ColliderBundle {
                    collider: Collider::cuboid(0.08, 0.1, 0.7),
                    ..Default::default()
                },
                c_groups: interactable_dynamic_body(),
                ..Default::default()
            })
            .insert(Interactable::HAMMER)
            .insert(AnimationPlayer::default());
    }
}

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ToolsPlugin::spawn_hammer.run_in_state(GameState::InOffice).run_if(NewLevel::has_triggered));
    }
}
