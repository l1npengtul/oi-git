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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum OfficeSpawnStates {
    LoadGltf,
    SpawnColliders,
}

pub struct OfficePlugin;

impl Plugin for OfficePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneLocations>()
            .init_resource::<OfficeEntities>()
            .add_enter_system(
                GameState::InOffice,
                spawn_office
                    .label(OfficeSpawnStates::LoadGltf)
                    .before(OfficeSpawnStates::SpawnColliders),
            )
            .add_enter_system(
                GameState::InOffice,
                spawn_extra_collider
                    .label(OfficeSpawnStates::SpawnColliders)
                    .after(OfficeSpawnStates::LoadGltf),
            )
            .add_exit_system(
                GameState::AssetLoading,
                SceneLocations::load_from_office_assets,
            );
    }
}

#[derive(Default)]
pub struct SceneLocations {
    pub locations: HashMap<&'static str, Transform>,
}

impl SceneLocations {
    pub fn load_from_office_assets(
        mut scene_locations: ResMut<SceneLocations>,
        office: Res<OfficeAssets>,
    ) {
        for (name, builder) in office.assets.iter() {
            if matches!(builder.kind, OfficeAssetKind::Point3D) {
                let mut proper_trans = builder.trans;
                proper_trans.scale = Vec3::ONE;

                scene_locations.locations.insert(name.clone(), proper_trans);
            }
        }
    }
}

pub struct OfficeAssets {
    pub assets: HashMap<&'static str, OfficeAssetBuilder>,
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
    Interactable,
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
            Interactable,
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
            Interactable => "interactable_",
            Point3D => "point3d_",
            RenderTarget => "render_target_",
            Normal => "",
            EmissiveNormal => "emissive_",
        }
    }
}

#[derive(Default)]
pub struct OfficeEntities {
    pub entities: HashMap<&'static str, Entity>,
}

fn leak_string(s: &String) -> &'static str {
    let b = s.clone().into_boxed_str();
    Box::leak(b)
}
