use crate::prelude::*;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    utils::HashMap,
};
use bevy_asset_loader::prelude::AssetCollection;

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
    pub name: String,
}


#[derive(Debug, Clone, Copy)]
pub enum OfficeAssetKind {
    Collider,
    Sensor,
    Dynamic,
    Point3D,
    RenderTarget,
    Normal,
}

impl OfficeAssetKind {
    pub fn from_str_prefix(s: &str) -> Self {
        use OfficeAssetKind::*;
        for kind in &[Collider, Sensor, Dynamic, Point3D, RenderTarget] {
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
        }
    }
}