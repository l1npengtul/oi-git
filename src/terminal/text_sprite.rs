use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy_asset_loader::prelude::AssetCollection;

use super::conv_cp437;

#[derive(AssetCollection)]
pub struct FontAtlas {
    #[asset(texture_atlas(
        tile_size_x = 32.,
        tile_size_y = 62.,
        columns = 16,
        rows = 16,
        padding_y = 2.5
    ))]
    #[asset(path = "fonts/mono-cp437.png")]
    pub atlas: Handle<TextureAtlas>,
}

pub const ATLAS_CHAR_W: f32 = 32.;
pub const ATLAS_CHAR_H: f32 = 64.;

pub struct TextSpritePlugin;

impl Plugin for TextSpritePlugin {
    fn build(&self, _: &mut App) {}
}

#[derive(Component, Debug, Clone)]
pub struct TextSprite {
    pub text: String,
    pub atlas: Handle<TextureAtlas>,
    pub scale: f32,
    pub chars: Vec<Entity>,
}

#[derive(Bundle)]
pub struct TextSpriteBundle {
    pub this: TextSprite,
    #[bundle]
    pub vis: VisibilityBundle,
    #[bundle]
    pub trans: TransformBundle,
}

#[derive(Component, Debug, Clone)]
pub struct TextSpriteChar;

#[derive(Bundle)]
pub struct TextSpriteCharBundle {
    this: TextSpriteChar,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub fn iter_row_col(s: &str) -> impl Iterator<Item = (usize, usize, char)> + '_ {
    s.lines()
        .enumerate()
        .flat_map(|(ri, col)| col.chars().enumerate().map(move |(ci, ch)| (ri, ci, ch)))
}

impl TextSprite {
    pub fn new(text: String, atlas: Handle<TextureAtlas>, scale: f32) -> Self {
        Self {
            text,
            atlas,
            scale,
            chars: Vec::new(),
        }
    }
    /// Spawns the text stored in the TextSpriteBuilder
    /// Takes closures to be applied to every entity spawned
    /// If further modifications are required
    pub fn spawn<ChildModifier, ParentModifier>(
        mut self,
        commands: &mut Commands,
        child_modifier: ChildModifier,
        parent_modifier: ParentModifier,
    ) where
        ChildModifier: FnMut(EntityCommands),
        ParentModifier: FnOnce(EntityCommands),
    {
        let mut parent = commands.spawn();
        parent.add_children(|builder| self.spawn_chars(builder, child_modifier, 0));
        parent.insert_bundle(TextSpriteBundle {
            this: self.clone(),
            vis: VisibilityBundle::default(),
            trans: TransformBundle::default(),
        });
        parent_modifier(parent)
    }

    /// `offset` is the number of chars to skip before spawning new ones
    pub fn spawn_chars<ChildModifier>(
        &mut self,
        builder: &mut ChildBuilder,
        mut child_modifier: ChildModifier,
        offset: usize,
    ) where
        ChildModifier: FnMut(EntityCommands),
    {
        let (w, h) = (self.scale * ATLAS_CHAR_W, self.scale * ATLAS_CHAR_H);
        assert!(
            self.len() >= offset,
            "cannot offset by more than the text's length"
        );
        for (ri, ci, ch) in iter_row_col(&self.text).skip(offset) {
            let child_commands = builder.spawn_bundle(TextSpriteCharBundle {
                this: TextSpriteChar,
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(conv_cp437::index_of(ch).unwrap()),
                    texture_atlas: self.atlas.clone(),
                    transform: Transform {
                        scale: Vec3::splat(self.scale),
                        translation: Vec3::new(ci as f32 * w, ri as f32 * -h, 0.0),
                        ..default()
                    },
                    ..default()
                },
            });
            self.chars.push(child_commands.id());
            child_modifier(child_commands);
        }
    }
    /// length disregarding linebreaks
    pub fn len(&self) -> usize {
        self.text.lines().map(|ln| ln.len()).sum()
    }

    pub fn add_str<ChildModifier>(
        &mut self,
        s: &str,
        commands: &mut Commands,
        parent: Entity,
        child_modifier: ChildModifier,
    ) where
        ChildModifier: FnMut(EntityCommands),
    {
        let offset = self.len();
        self.text.push_str(s);
        commands
            .entity(parent)
            .add_children(|builder| self.spawn_chars(builder, child_modifier, offset))
    }

    pub fn add_multiline_str(&mut self, s: &str, commands: &mut Commands, parent: Entity) {
        for ln in s.lines() {
            self.add_str(s, commands, parent, |_| {})
        }
    }

    pub fn remove_top_lines(&mut self, commands: &mut Commands, count: usize) {}

    pub fn push_newline(&mut self) {
        self.text.push('\n');
    }

    pub fn pop(&mut self, commands: &mut Commands) {
        if self.text.pop() != Some('\n') {
            self.chars
                .pop()
                .into_iter()
                .for_each(|entity| commands.entity(entity).despawn());
        }
    }
}
