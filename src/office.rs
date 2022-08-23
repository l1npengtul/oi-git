use crate::prelude::*;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    utils::HashMap,
};
use bevy_asset_loader::prelude::AssetCollection;

mod load;
pub use load::*;
mod spawn;
pub use spawn::*;

pub struct OfficePlugin;

impl Plugin for OfficePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneLocations>()
            .init_resource::<OfficeEntities>()
            .add_enter_system(GameState::MainMenu, spawn_office)
            .add_exit_system(
                GameState::AssetLoading,
                SceneLocations::load_from_office_assets,
            );
    }
}

#[derive(Default)]
pub struct SceneLocations {
    pub locations: HashMap<String, Transform>,
}

impl SceneLocations {
    pub fn load_from_office_assets(
        mut scene_locations: ResMut<SceneLocations>,
        office: Res<OfficeAssets>,
    ) {
        for (name, builder) in office.assets.iter() {
            if matches!(builder.kind, OfficeAssetKind::Point3D) {
                scene_locations
                    .locations
                    .insert(name.clone(), builder.trans);
            }
        }
    }
}

pub struct OfficeAssets {
    pub assets: HashMap<String, OfficeAssetBuilder>,
}

#[derive(AssetCollection)]
pub struct OfficeScene {
    #[asset(path = "office/office.glb")]
    pub scene: Handle<Gltf>,
}

pub struct OfficeAssetBuilder {
    pub kind: OfficeAssetKind,
    pub node: Handle<GltfNode>,
    pub mesh: Handle<GltfMesh>,
    pub collider_mesh: Option<Handle<Mesh>>,
    pub trans: Transform,
}

#[derive(Debug, Clone, Copy)]
pub enum OfficeAssetKind {
    Collider,
    Sensor,
    Dynamic,
    Point3D,
    RenderTarget,
    Normal,
    EmissiveNormal,
}

impl OfficeAssetKind {
    pub fn from_str_prefix(s: &str) -> Self {
        use OfficeAssetKind::*;
        for kind in &[
            Collider,
            Sensor,
            Dynamic,
            Point3D,
            RenderTarget,
            EmissiveNormal,
        ] {
            if s.starts_with(kind.prefix_of()) {
                return *kind;
            }
        }
        Normal
    }
    pub fn prefix_of(&self) -> &'static str {
        use OfficeAssetKind::*;
        match self {
            Collider => "collider_",
            Sensor => "sensor_",
            Dynamic => "dynamic_",
            Point3D => "point3d_",
            RenderTarget => "render_target_",
            Normal => "",
            EmissiveNormal => "emissive_",
        }
    }
}

#[derive(Default)]
pub struct OfficeEntities {
    map: HashMap<String, Entity>
}
