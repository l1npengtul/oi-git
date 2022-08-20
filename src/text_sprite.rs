use crate::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

mod conv_cp437;

#[derive(AssetCollection)]
pub struct FontAtlas {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 64., columns = 16, rows = 16))]
    #[asset(path = "fonts/mono-cp437.png")]
    pub atlas: Handle<TextureAtlas>,
}

pub struct TextSpritePlugin;

impl Plugin for TextSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_all_chars));
    }
}

fn spawn_all_chars(mut comands: Commands, font: Res<FontAtlas>) {
    let string = " !\"#$%&\\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let mut transform = Transform {
        scale: Vec3::splat(0.5),
        translation: Vec3::new(-crate::WIDTH/2.0, 0.0, 0.0),
        ..default()
    };
    for ch in string.chars() {
        let idx = FontAtlas::index_of(ch);
        comands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(idx),
            texture_atlas: font.atlas.clone(),
            transform,
            ..default()
        });
        transform.translation.x += 16.;
    }
}