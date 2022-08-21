#![doc = include_str!("../README.md")]

use crate::main_scene::{set_up_2d, setup_main_scene, spawn_camera, TargetImage};
use crate::prelude::*;

mod asset;
mod debug;
mod main_scene;
mod prelude;
mod state;
mod utils;
mod terminal;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const INITIAL_STATE: GameState = GameState::MainMenu;

fn main() {
    let mut app = App::new();
    app.insert_resource(utils::window_descriptor(WIDTH, HEIGHT))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(asset::AssetLoaderPlugin {
            initial_state: INITIAL_STATE,
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .init_resource::<TargetImage>()
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_plugin(terminal::TerminalPlugin)
        .add_plugin(main_scene::MainScenePlugin)
        .add_startup_system(spawn_camera);
    app.run();
}
