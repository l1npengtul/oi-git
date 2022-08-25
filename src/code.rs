use crate::level::Levels;
use crate::terminal::{ATLAS_CHAR_H, ATLAS_CHAR_W};
use crate::{
    collider::{ColliderBundle, PhysicsBundle},
    prelude::{phys::*, *},
    terminal::{TextSprite, TextSpriteBundle},
};
use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    text::FontAtlas,
};
use bevy_asset_loader::asset_collection::AssetCollection;

pub struct CodePlugin;

impl Plugin for CodePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Debug, Clone)]
pub struct LineOfCode {
    pub diff: Diff,
    pub color: CodeColor,
    pub code: String,
}

#[derive(AssetCollection)]
pub struct LoCScene {
    #[asset(path = "tools_and_viewmodels/plank_notexture.glb")]
    pub gltf: Handle<Gltf>,
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

pub struct LoCMesh {
    mesh: Handle<GltfMesh>,
}

impl FromWorld for LoCMesh {
    fn from_world(world: &mut World) -> Self {
        utils::build_world_access_macros!(world, res, assets);
        let scene = res!(LoCScene);
        let assets = assets!(Gltf).get(&scene.gltf).unwrap();
        let plank = assets.named_meshes.get("plank").unwrap();

        Self {
            mesh: plank.clone(),
        }
    }
}

#[derive(Component)]
pub struct LoCCamera;

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
pub const SCALE: f32 = 4.;

fn spawn_level(mut commands: Commands, levels: Res<Levels>, font: Res<FontAtlas>) {
    let mut pos = CODE_SPRITE_OFFSET;
    for (i, loc) in levels.levels[levels.current].code.iter().enumerate() {
        let mut text_sprite = TextSprite::new(loc.code.clone(), font.texture_atlas.clone(), SCALE);
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

        let camera = commands.spawn_bundle(Camera2dBundle {
            // i cant remember if bevy sprites start from center of transform
            // or one of the coners (this is stuff that assumes its centered on the transform)
            transform: Transform::from_xyz(
                pos.x - (ATLAS_CHAR_W * 0.5) + (CODE_LINE_LENGTH as f32 * ATLAS_CHAR_W / 2.),
                pos.y + ATLAS_CHAR_H * 0.5 + ATLAS_CHAR_H * i as f32,
                0.,
            ),
            ..default()
        }).insert(LoCCamera);

        pos.y -= 100.;
    }
}
