use crate::prelude::*;
use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection)]
pub struct MainSceneAssets {
    #[asset(path = "office/terminal.glb#Scene0")]
    terminal: Handle<Scene>,
    // #[asset(path = "office/screen/rendertarget.glb#Mesh0")]
    // render_target: Handle<Scene>
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut targetmat: ResMut<TargetImage>,
    main_scene_assets: Res<MainSceneAssets>,
) {
    let display = main_scene_assets.terminal.clone();

    commands.spawn_bundle(SceneBundle {
        scene: display,
        transform: Default::default(),
        global_transform: Default::default(),
        visibility: Default::default(),
        computed_visibility: Default::default(),
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

    let cube_handle = meshes.add(Mesh::from(shape::Plane { size: 10.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..Default::default()
    });

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.

    // The cube that will be rendered to the texture.
    commands.spawn_bundle(PbrBundle {
        mesh: cube_handle,
        material: cube_material_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

pub fn set_up_2d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut c_materials: ResMut<Assets<ColorMaterial>>,
    targetmat: Res<TargetImage>,
) {
    commands.spawn_bundle(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(targetmat.0.clone()),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: c_materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
}
