use crate::prelude::*;
use crate::viewmodel::ViewModel;
use bevy::ecs::query::QuerySingleError;

pub(crate) struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    #[cfg(not(debug_assertions))]
    fn build(&mut self, _: &mut bevy::app::PluginGroupBuilder) {}

    #[cfg(debug_assertions)]
    fn build(&mut self, #[allow(unused)] group: &mut bevy::app::PluginGroupBuilder) {
        // group
        //     .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //     .add(bevy::diagnostic::EntityCountDiagnosticsPlugin);

        #[cfg(not(feature = "editor"))]
        group.add(bevy::diagnostic::LogDiagnosticsPlugin::default());
        #[cfg(feature = "editor")]
        group.add(bevy_editor_pls::prelude::EditorPlugin);
        #[cfg(feature = "show_collider")]
        group.add(bevy_rapier3d::prelude::RapierDebugRenderPlugin {
            depth_test: false,
            style: bevy_rapier3d::prelude::DebugRenderStyle::default(),
            mode: bevy_rapier3d::prelude::DebugRenderMode::default(),
        });
    }
}

pub fn viewmodel_holding(vmh: Query<&ViewModel>) {
    let vm = match vmh.get_single() {
        Ok(o) => o,
        Err(_) => return,
    };
    println!("holding: {:?}", vm.holding());
}
