#![doc = include_str!("../README.md")]

use crate::prelude::*;
use crate::utils::ColliderData;
use bevy_rapier3d::plugin::RapierPhysicsPlugin;

mod asset;
mod config;
mod debug;
mod grab_cursor;
mod main_scene;
mod physics;
mod player;
mod prelude;
mod state;
mod terminal;
mod utils;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const INITIAL_STATE: GameState = GameState::MainMenu;

fn main() {
    let mut app = App::new();
    app.insert_resource(utils::window_descriptor(WIDTH, HEIGHT))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(config::ConfigPlugin::default())
        .add_plugin(asset::AssetLoaderPlugin {
            initial_state: INITIAL_STATE,
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_plugin(terminal::TerminalPlugin)
        .add_plugin(main_scene::MainScenePlugin)
        .add_plugin(RapierPhysicsPlugin::<ColliderData>::default())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(grab_cursor::GrabCursorPlugin);
    app.run();
}
