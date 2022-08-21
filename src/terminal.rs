use crate::prelude::*;

pub mod conv_cp437;
mod text_sprite;
pub use text_sprite::*;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TextSpritePlugin);
    }
}
