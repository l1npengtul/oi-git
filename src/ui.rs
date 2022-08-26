use crate::{
    App, Commands, Font, FromWorld, GameState, Handle, Plugin, Text, Text2dBundle, TextStyle,
    Transform, World,
};
use iyes_loopless::prelude::AppLooplessStateExt;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, spawn_txt);
    }
}

pub fn spawn_txt(mut commands: Commands) {
    commands.spawn().insert_bundle(Text2dBundle {
        text: Text::from_section(
            "helo wrld",
            TextStyle {
                font_size: 10.0,
                ..Default::default()
            },
        ),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    });
}
