#![doc = include_str!("../README.md")]

use bevy::prelude::*;

mod debug;
mod vc_bevy_jam_2;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!(
                "{} - v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(vc_bevy_jam_2::VcBevyJam2Plugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}
