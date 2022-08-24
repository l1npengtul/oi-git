use crate::*;
use super::*;

impl TerminalInput {
    pub fn spawn(
        mut commands: Commands,
        font: Res<FontAtlas>,
        target: Res<TerminalScreenTarget>,
        office: Res<OfficeAssets>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        level: Res<LevelsCode>
    ) {
        let prompt = TextSprite::new("input: ".to_string(), font.atlas.clone(), 1.0);
        prompt.spawn(
            &mut commands,
            |_| {},
            |mut parent| {
                parent.insert(TerminalInput);
                parent.insert(Transform::from_xyz(
                    (ATLAS_CHAR_W - TERM_W) / 2.0,
                    (TERM_H - ATLAS_CHAR_H) / 2.,
                    0.,
                ));
            },
        );


        // spawning the terminal render stuff
        let target_material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(target.image.clone()),
            reflectance: 0.02,
            unlit: false,
            ..Default::default()
        });

        let builder = office.assets.get("render_target").unwrap();

        // The cube that will be rendered to the texture.
        commands.spawn_bundle(MaterialMeshBundle {
            mesh: builder.collider_mesh.clone().unwrap(),
            material: target_material_handle,
            transform: builder.trans,
            ..Default::default()
        });
    }
}
