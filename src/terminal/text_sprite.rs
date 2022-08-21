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
    fn build(&self, app: &mut App) {
        app.add_enter_system(
            GameState::MainMenu, spawn_with_builder,
        );
    }
}

fn spawn_with_builder(mut commands: Commands, font: Res<FontAtlas>) {
    info!("spawning");
    let all_chars = " a burger\nat night";

    let builder = TextSpriteBuilder {
        text: all_chars.to_string(),
        atlas: font.atlas.clone(),
        scale: 0.3,
    };
    builder.spawn(&mut commands, |_| {}, |_| {});
}

#[derive(Component)]
struct TextSprite;

pub struct TextSpriteBuilder {
    pub text: String,
    pub atlas: Handle<TextureAtlas>,
    pub scale: f32,
}

impl TextSpriteBuilder {
    /// Spawns the text stored in the TextSpriteBuilder
    /// Takes closures to be applied to every entity spawned
    /// If further modifications are required
    pub fn spawn<ChildModifier, ParentModifier>(
        &self,
        commands: &mut Commands,
        mut child: ChildModifier,
        parent: ParentModifier,
    ) where
        ChildModifier: FnMut(EntityCommands),
        ParentModifier: FnOnce(EntityCommands),
    {
        let mut parent_entity = commands.spawn_bundle(VisibilityBundle::default());
        parent_entity.insert_bundle(TransformBundle::default());
        parent_entity.insert(TextSprite);
        let (w, h) = (self.scale * ATLAS_CHAR_W, self.scale * ATLAS_CHAR_H);

        parent_entity.add_children(|builder| {
            for (ln, i, ch) in self
                .text
                .lines()
                .enumerate()
                .flat_map(|(ln, s)| s.chars().enumerate().map(move |(i, ch)| (ln, i, ch)))
            {
                let child_entity = builder.spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(conv_cp437::index_of(ch)),
                    texture_atlas: self.atlas.clone(),
                    transform: Transform {
                        scale: Vec3::splat(self.scale),
                        translation: Vec3::new(i as f32 * w, ln as f32 * -h, 0.0),
                        ..default()
                    },
                    ..Default::default()
                });
                child(child_entity);
            }
        });

        parent(parent_entity)
    }
}
