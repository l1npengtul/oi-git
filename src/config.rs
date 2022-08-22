use crate::prelude::*;

macro_rules! impl_plugin_config {
    ($($field:ident),* $(,)?) => {
        impl Plugin for ConfigPlugin {
            fn build(&self, app: &mut App) {
                app$(.insert_resource(self.$field.clone()))*;

            }
        }
    };
}

// NOTE: whenever adding a new field,
// also add one to macro below
pub struct ConfigPlugin {
    player: PlayerConfig,
}

impl_plugin_config!(player);

#[derive(Debug, Clone)]
pub struct PlayerConfig {
    pub mouse_sens: f32,
    pub mvmnt_speed: f32,
}

pub const DEFAULT_CONFIG: ConfigPlugin = ConfigPlugin {
    player: PlayerConfig {
        mouse_sens: 0.00012,
        mvmnt_speed: 12.,
    },
};

impl Default for ConfigPlugin {
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}
