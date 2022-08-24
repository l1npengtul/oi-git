use crate::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection)]
pub struct HammerModel {
    #[asset(path = "assets/tools_and_viewmodels/hammer.glb#Scene0")]
    pub hammer_scene: Handle<Scene>,
    #[asset(path = "assets/tools_and_viewmodels/hammer.glb#Animation0")]
    pub swing_animation: Handle<AnimationClip>,
}

#[derive(Component)]
pub struct ViewModel {}

#[derive(Bundle)]
pub struct ViewModelBundle {
    #[bundle]
    pub transform: TransformBundle,
    pub viewmodel: ViewModel,
}
// TODO: Polish: Render on top
//
// #[derive(AsBindGroup, TypeUuid, Clone, Default)]
// #[uuid = "7494888b-c082-457b-aacf-517228cc0c23"]
// pub struct ViewModelMaterial {
//     standard_material: StandardMaterial,
// }
//
// impl From<StandardMaterial> for ViewModelMaterial {
//     fn from(sm: StandardMaterial) -> Self {
//         ViewModelMaterial {
//             standard_material: sm,
//         }
//     }
// }
//
// impl From<ViewModelMaterial> for StandardMaterial {
//     fn from(vmm: ViewModelMaterial) -> Self {
//         vmm.standard_material
//     }
// }
//
// impl Material for ViewModelMaterial {}
