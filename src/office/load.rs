use crate::prelude::*;
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    utils::HashMap,
};
use super::{OfficeAssets, OfficeScene, OfficeAssetBuilder, OfficeAssetKind};

impl FromWorld for OfficeAssets {
    fn from_world(world: &mut World) -> Self {
        // fancy shortcut to get resources
        // from the world
        utils::build_world_access_macros!(world, res, assets);
        let mut assets = HashMap::new();
        let office_scene = res!(OfficeScene);
        let office_gltf = assets!(Gltf).get(&office_scene.scene).unwrap();
        for (name, node_handle) in &office_gltf.named_nodes {
            let warn = || warn!("skipping {name}");
            let node = utils::unwrap_or_continue!(assets!(GltfNode).get(node_handle); else warn());
            let mesh_handle = utils::unwrap_or_continue!(&node.mesh; else warn());
            let mesh =
                utils::unwrap_or_continue!(assets!(GltfMesh).get(mesh_handle); else warn());
            let asset_kind = OfficeAssetKind::from_str_prefix(name);

            assets.insert(name.clone(), OfficeAssetBuilder {
                kind: asset_kind,
                name: name.clone(),
                node: node_handle.clone(),
                mesh: mesh_handle.clone(),
                collider_mesh: mesh.primitives.get(0).map(|p| p.mesh.clone()),
            });
        }

        Self { assets }
    }
}
