use crate::{
    collider::{ColliderBundle, PhysicsBundle},
    level::Levels,
    phys::group::collide::dynamic_body,
    prelude::{phys::*, *},
    terminal::{FontAtlas, TextSprite, TextSpriteBundle, ATLAS_CHAR_H, ATLAS_CHAR_W},
};
use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
};
use bevy_asset_loader::asset_collection::AssetCollection;

pub struct CodePlugin;

impl Plugin for CodePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, spawn_level);
    }
}

#[derive(Component, Debug, Clone)]
pub struct LineOfCode {
    pub diff: Diff,
    pub color: CodeColor,
    pub code: String,
}

#[derive(AssetCollection)]
pub struct LoCScene {
    #[asset(path = "tools_and_viewmodels/plank_notexture.glb#Mesh0/Primitive0")]
    pub gltf: Handle<Mesh>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    Pos,
    Neg,
    Eq,
    Rem,
}

impl Diff {
    pub fn prefix(&self) -> &'static str {
        match self {
            Diff::Pos => "++",
            Diff::Neg => "--",
            Diff::Eq => "==",
            Diff::Rem => "!!",
        }
    }

    pub fn from_line(s: &str) -> Self {
        macro_rules! match_starts_with_prefix {
            ($s:expr => { $($variant:expr),* $(,)? }) => {
                match $s {
                    $(s if s.starts_with($variant.prefix()) => $variant,)*
                    s => panic!("prefix not found in line: {s}")
                }
            };
        }
        match_starts_with_prefix!(s => {
            Diff::Pos,
            Diff::Neg,
            Diff::Eq,
            Diff::Rem,
        })
    }

    pub fn to_color(&self) -> CodeColor {
        use CodeColor::*;
        use Diff::*;
        match self {
            Rem => None,
            Neg => Red,
            Pos => Green,
            Eq => Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeColor {
    Green,
    Red,
    Normal,
    None,
}

// // FIXME: extract the actual parts of the scene needed for the loc here
// pub struct LoCMesh {
//     // shouldn't be an option
//
// }

// impl FromWorld for LoCMesh {
//     fn from_world(world: &mut World) -> Self {
//         utils::build_world_access_macros!(world, res, assets);
//         let scene = res!(LoCScene);
//         let assets = assets!(Mesh).get(&scene.gltf).unwrap();
//
//         Self { mesh: assets }
//     }
// }

#[derive(Component)]
pub struct LoCCamera;

#[derive(Component)]
pub struct LoCBlock {
    pub line_of_code: String,
}

/// Spawn a separate camera
#[derive(Bundle)]
pub struct LoCSpriteBundle {
    loc: LineOfCode,
    #[bundle]
    text: TextSpriteBundle,
}

// everything is on the same 2D canvas
// so move the texts far apart
pub const CODE_SPRITE_OFFSET: Vec3 = Vec3::from_array([0., -2000., 0.]);
pub const CODE_LINE_LENGTH: usize = 60;
pub const SCALE: f32 = 1.;

fn spawn_level(
    mut commands: Commands,
    levels: Res<Levels>,
    font: Res<FontAtlas>,
    locscene: Res<LoCScene>,
    meshes: Res<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut pos = CODE_SPRITE_OFFSET;
    for (i, loc) in levels.levels[levels.current].code.iter().enumerate() {
        let mut text_sprite = TextSprite::new(loc.code.clone(), font.atlas.clone(), SCALE);
        let mut text = commands.spawn();

        text.add_children(|builder| text_sprite.spawn_chars(builder, |_| {}, 0));

        text.insert_bundle(LoCSpriteBundle {
            loc: loc.clone(),
            text: TextSpriteBundle {
                this: text_sprite,
                vis: default(),
                trans: TransformBundle::from_transform(Transform::from_translation(pos)),
            },
        });

        let size = Extent3d {
            width: 3300,
            height: 300,
            ..Default::default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::bevy_default(),
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

        commands
            .spawn_bundle(Camera2dBundle {
                camera: Camera {
                    priority: 3,
                    target: RenderTarget::Image(image_handle.clone()),
                    ..Default::default()
                },
                // i cant remember if bevy sprites start from center of transform
                // or one of the coners (this is stuff that assumes its centered on the transform)
                transform: Transform::from_xyz(
                    pos.x - (ATLAS_CHAR_W * 0.5) + (CODE_LINE_LENGTH as f32 * ATLAS_CHAR_W / 2.),
                    pos.y + ATLAS_CHAR_H * 0.5 + ATLAS_CHAR_H * i as f32,
                    0.,
                ),
                ..default()
            })
            .insert(LoCCamera)
            .insert(UiCameraConfig { show_ui: false });

        pos.y -= ATLAS_CHAR_H * SCALE * 2.0;

        let mdl_trans = Transform::from_xyz(0.0, 0.5, -2.0);
        let mdl_trans = mdl_trans.with_scale(Vec3::new(0.15, 0.035, 1.0));
        // spawn the mesh
        commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: locscene.gltf.clone(),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(image_handle),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(LoCBlock {
                line_of_code: loc.code.clone(),
            })
            .insert_bundle(PhysicsBundle {
                body: RigidBody::Dynamic,
                collider: ColliderBundle {
                    collider: Collider::from_bevy_mesh(
                        meshes.get(&locscene.gltf).unwrap(),
                        &ComputedColliderShape::TriMesh,
                    )
                    .unwrap(),
                    ..Default::default()
                },
                c_groups: dynamic_body(),
                ..Default::default()
            })
            .insert(Ccd::enabled())
            .insert_bundle(TransformBundle {
                local: mdl_trans,
                ..Default::default()
            });
    }
}
