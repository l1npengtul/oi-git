use crate::prelude::*;
use bevy_asset_loader::prelude::*;

/// Goes before DefaultPlugins
pub struct AssetLoaderPlugin {
    pub initial_state: GameState,
}

macro_rules! impl_plugin_with_assets {
    (
        static: {$($static_asset:ty),* $(,)?}
        dynamic: {$(($($file:literal),* $(,)?) => $dynamic_asset:ty),* $(,)?}
        init: {$($init_resource:ty),* $(,)?}
    ) => {
        impl Plugin for AssetLoaderPlugin {
            fn build(&self, app: &mut App) {
                app.add_loopless_state(GameState::AssetLoading)
                    .add_loading_state(
                    LoadingState::new(GameState::AssetLoading)
                        .continue_to_state(self.initial_state.clone())
                        $(
                            .with_collection::<$static_asset>()
                        )*
                        $(
                            .with_dynamic_collections::<$dynamic_asset>(vec![$($file)*])
                        )*
                        $(
                            .init_resource::<$init_resource>()
                        )*
                )
                .add_state(GameState::AssetLoading);
            }
        }
    };
}

impl_plugin_with_assets!(
    static: {
        crate::terminal::FontAtlas,
        crate::office::OfficeScene,
    }
    dynamic: {}
    init: {
        crate::office::TerminalScreenTarget,
        crate::office::SceneLocations,
        crate::office::OfficeAssets,
    }
);
