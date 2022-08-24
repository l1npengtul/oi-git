use crate::prelude::*;
use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
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

#[derive(Component)]
pub struct TerminalCamera;

impl TerminalScreenTarget {
    pub fn set_up_2d(mut commands: Commands, target: Res<TerminalScreenTarget>) {
        commands
            .spawn_bundle(Camera2dBundle {
                camera: Camera {
                    target: RenderTarget::Image(target.image.clone()),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(TerminalCamera);
    }
}
