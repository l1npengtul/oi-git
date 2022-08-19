#![doc = include_str!("../README.md")]

use bevy::prelude::*;

mod debug;
mod prelude;
mod state;
mod utils;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);



fn main() {
    let mut app = App::new();
    app
        .insert_resource(utils::window_descriptor(WIDTH, HEIGHT))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_startup_system(spawn_camera);
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
