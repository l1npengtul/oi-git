use crate::prelude::*;
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
pub struct MainSceneAssets {
    #[asset(path = "office/terminal.glb#Scene0")]
    terminal: Handle<Scene>,
    #[asset(path = "office/screen/rendertarget.glb#Node0")]
    render_target: Handle<GltfNode>,
}

pub struct MainScenePlugin;

impl Plugin for MainScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, setup_main_scene)
        .add_enter_system(GameState::MainMenu, set_up_2d);
    }
}

#[derive(Default)]
pub struct TargetImage(Handle<Image>);

pub fn setup_main_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut targetmat: ResMut<TargetImage>,
    gltf_mesh: Res<Assets<GltfMesh>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    main_scene_assets: Res<MainSceneAssets>,
) {
    let display = main_scene_assets.terminal.clone();

    commands.spawn_bundle(SceneBundle {
        scene: display,
        ..Default::default()
    });

    // render_target in MainSceneAssets
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
    targetmat.0 = image_handle.clone();

    let (target_transform, target_mesh) = {
        let node = gltf_nodes.get(&main_scene_assets.render_target).unwrap();
        let mesh = gltf_mesh
            .get(node.mesh.as_ref().unwrap())
            .unwrap()
            .primitives[0]
            .mesh
            .clone();
        (node.transform, mesh)
    };
    let target_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..Default::default()
    });

    // The cube that will be rendered to the texture.
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: target_mesh,
        material: target_material_handle,
        transform: target_transform,
        ..Default::default()
    });
}

pub fn set_up_2d(mut commands: Commands, targetmat: Res<TargetImage>) {
    commands.spawn_bundle(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(targetmat.0.clone()),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });
}
