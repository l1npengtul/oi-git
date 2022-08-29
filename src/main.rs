#![doc = include_str!("../README.md")]

use crate::prelude::*;
use crate::score::TotalPoints;
use crate::utils::ColliderData;
use bevy::pbr::PointLightShadowMap;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::plugin::RapierPhysicsPlugin;

mod asset;
mod audio;
mod code;
mod collider;
mod config;
mod debug;
mod grab_cursor;
mod interactable;
mod level;
mod office;
mod player;
mod prelude;
mod score;
mod state;
mod terminal;
mod tools;
mod ui;
mod utils;
mod viewmodel;

const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 540.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const INITIAL_STATE: GameState = GameState::MainMenu;

const BRIGHTNESS: f32 = 0.2;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    app.insert_resource(utils::window_descriptor(WIDTH, HEIGHT))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(config::ConfigPlugin::default())
        .add_plugin(asset::AssetLoaderPlugin {
            initial_state: INITIAL_STATE,
        })
        .add_plugin(level::LevelPlugin)
        .insert_resource(AmbientLight {
            color: Color::rgb(0.79, 0.73, 0.53),
            brightness: BRIGHTNESS,
        })
        .insert_resource(PointLightShadowMap { size: 512 })
        .insert_resource(TotalPoints { sum: 0., total: 0. })
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_plugin(terminal::TerminalPlugin)
        .add_plugin(office::OfficePlugin)
        .add_plugin(RapierPhysicsPlugin::<ColliderData>::default())
        .add_plugin(player::PlayerPlugin)
        .add_plugin(grab_cursor::GrabCursorPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(code::CodePlugin)
        .add_plugin(tools::ToolsPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(audio::SusdioPlugin);
    app.run();
}
