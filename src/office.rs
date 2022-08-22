use crate::prelude::{phys::*, *};
use crate::utils::{ColliderType, EName};
use bevy::gltf::Gltf;
use bevy::utils::HashMap;
use bevy::{
    gltf::{GltfMesh, GltfNode},
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection)]
pub struct OfficeScene {
    #[asset(path = "office/office_proto_noceil.glb")]
    scene: Handle<Gltf>,
}

pub struct OfficePlugin;

impl Plugin for OfficePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::MainMenu,
            SystemSet::new()
                .with_system(setup_main_scene)
                .with_system(set_up_2d),
        );
    }
}

pub struct TerminalScreenTarget {
    pub image: Handle<Image>,
}

impl FromWorld for TerminalScreenTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        // render_target in OfficeAssets
        let size = Extent3d {
            width: 1280,
            height: 960,
            ..Default::default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..Default::default()
        };

        // fill image.data with zeroes
        image.resize(size);

        let image_handle = images.add(image);
        Self {
            image: image_handle,
        }
    }
}

pub struct SceneLocations {
    pub locations: HashMap<String, Transform>,
}

impl FromWorld for SceneLocations {
    fn from_world(_world: &mut World) -> Self {
        Self {
            locations: Default::default(),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn setup_main_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_locations: ResMut<SceneLocations>,
    target: Res<TerminalScreenTarget>,
    mesh: Res<Assets<Mesh>>,
    gltf_mesh: Res<Assets<GltfMesh>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    gltf: Res<Assets<Gltf>>,
    office_scene: Res<OfficeScene>,
) {
    // HEY THERE FUTURE DUMBASS
    // LEVELS ARE LIKE THIS
    // starts with "collider_": Becomes static body collider
    // starts with "sensor_": Becomes a sensor with name
    // starts with "dynamic_": Dynamic Object, format: `dynamic_{f32: Friction}_{f32: Restitution}_{name}`
    // anything else => Becomes a normal mesh
    let default_material = materials.add(StandardMaterial::default());
    let office_gltf = gltf.get(&office_scene.scene).unwrap();

    for (node_name, gl_node) in &office_gltf.named_nodes {
        let warn = || warn!("skipping {node_name}");
        let node = utils::unwrap_or_continue!(gltf_nodes.get(gl_node); else warn());
        let n_gl_mesh_handle = utils::unwrap_or_continue!(&node.mesh; else warn());
        let n_gl_mesh = utils::unwrap_or_continue!(gltf_mesh.get(n_gl_mesh_handle); else warn());

        let node_transform = node.transform;

        if node_name.starts_with("collider_") {
            let c_mesh = mesh.get(&n_gl_mesh.primitives[0].mesh).unwrap();

            commands
                .spawn()
                .insert(RigidBody::Fixed)
                .insert(Collider::from_bevy_mesh(c_mesh, &ComputedColliderShape::TriMesh).unwrap())
                .insert(ColliderType::Static)
                .insert(EName {
                    id: node_name.clone(),
                })
                .insert_bundle(TransformBundle::from_transform(node_transform));
        } else if node_name.starts_with("sensor_") {
            let c_mesh = mesh.get(&n_gl_mesh.primitives[0].mesh).unwrap();

            commands
                .spawn()
                .insert(Collider::from_bevy_mesh(c_mesh, &ComputedColliderShape::TriMesh).unwrap())
                .insert(Sensor)
                .insert(ColliderType::Sensor)
                .insert(EName {
                    id: node_name.clone(),
                })
                .insert_bundle(TransformBundle::from_transform(node_transform));
        } else if node_name.starts_with("dynamic_") {
            // parse the name :skull:
            let name_sections = node_name.split('_').collect::<Vec<&str>>();
            let friction = name_sections[1].parse::<f32>().unwrap();
            let restitution = name_sections[2].parse::<f32>().unwrap();
            let name = format!("dynamic_{}", name_sections[3]);
            let c_mesh = mesh.get(&n_gl_mesh.primitives[0].mesh).unwrap();
            let c_mat = &n_gl_mesh.primitives[0].material;

            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(Collider::from_bevy_mesh(c_mesh, &ComputedColliderShape::TriMesh).unwrap())
                .insert(Friction::new(friction))
                .insert(Restitution::new(restitution))
                .insert(ColliderType::Dynamic)
                .insert(EName { id: name })
                .insert_bundle(TransformBundle::from_transform(node_transform))
                .insert_bundle(PbrBundle {
                    mesh: n_gl_mesh.primitives[0].mesh.clone(),
                    material: c_mat.clone().unwrap_or_else(|| default_material.clone()),
                    transform: node_transform,
                    ..Default::default()
                });
        } else if node_name.starts_with("point3d_") {
            scene_locations
                .locations
                .insert(node_name.clone(), node_transform);
        } else if node_name.starts_with("render_target") {
            let target_material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(target.image.clone()),
                reflectance: 0.02,
                unlit: false,
                ..Default::default()
            });

            // The cube that will be rendered to the texture.
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: n_gl_mesh.primitives[0].mesh.clone(),
                material: target_material_handle,
                transform: node_transform,
                ..Default::default()
            });
        } else {
            for meshie_handlies in &n_gl_mesh.primitives {
                commands.spawn_bundle(PbrBundle {
                    mesh: meshie_handlies.mesh.clone(),
                    material: meshie_handlies
                        .material
                        .clone()
                        .unwrap_or_else(|| default_material.clone()),
                    transform: node_transform,
                    ..Default::default()
                });
            }
        }
    }
}

pub fn set_up_2d(mut commands: Commands, target: Res<TerminalScreenTarget>) {
    commands.spawn_bundle(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(target.image.clone()),
            ..Default::default()
        },
        ..Default::default()
    });
}
