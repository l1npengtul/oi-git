use bevy::prelude::*;

pub struct VcBevyJam2Plugin;

impl Plugin for VcBevyJam2Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(rotate);
    }
}

#[derive(Debug, Component, Default)]
struct VcBevyJam2Component;

#[derive(Bundle, Default)]
struct VcBevyJam2Bundle {
    pub example: VcBevyJam2Component,
    #[bundle]
    pub scene: SceneBundle,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    use std::f32::consts::TAU;
    commands
        .spawn_bundle(VcBevyJam2Bundle {
            scene: SceneBundle {
                scene: asset_server.load("cube.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    22.5 * TAU / 360.0,
                    45.0 * TAU / 360.0,
                    0.0,
                )),
                ..default()
            },
            ..default()
        });
}

fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<VcBevyJam2Component>>) {
    use std::f32::consts::TAU;
    for mut transform in &mut transforms {
        transform.rotate_z(45.0 * TAU / 360.0 * time.delta_seconds());
    }
}
