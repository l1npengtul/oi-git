#![doc = include_str!("../README.md")]

use crate::main_scene::{set_up_2d, setup_main_scene, TargetImage};
use crate::prelude::*;

mod debug;
mod main_scene;
mod prelude;
mod state;
mod utils;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    let mut app = App::new();
    app.insert_resource(utils::window_descriptor(WIDTH, HEIGHT))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .init_resource::<TargetImage>()
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_plugin(state::StatePlugin {
            initial: GameState::MainMenu,
        })
        .add_startup_system(setup_main_scene)
        .add_startup_system(set_up_2d)
        .add_startup_system(spawn_camera);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });
}
