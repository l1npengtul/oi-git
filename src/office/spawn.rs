use super::{OfficeAssetBuilder, OfficeAssetKind, OfficeAssets, OfficeEntities};
use crate::collider::{ColliderBundle, PhysicsBundle};
use crate::interactable::Interactable;
use crate::office::SceneLocations;
use crate::phys::group::collide::{all, sensor, static_body};
use crate::prelude::{phys::*, utils::*, *};
use bevy::ecs::system::SystemParam;
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use std::f32::consts::FRAC_PI_4;

pub fn spawn_extra_collider(mut commands: Commands, locations: Res<SceneLocations>) {
    // ceiling
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(20.0, 10.0, 20.0),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0.0, -10.0, 0.0,
        )));
    // floor
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(20.0, 10.0, 20.0),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0.0, 12.9, 0.0,
        )));
    // spawndesk
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(1.2, 1.1, 1.05),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(
                locations
                    .locations
                    .get("point3d_spawndesk")
                    .unwrap()
                    .translation,
            ),
        ));
    // wall1
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(200.0, 100.0, 11.6),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(
            locations
                .locations
                .get("point3d_wall1")
                .unwrap()
                .with_scale(Vec3::ONE),
        )); // wall1
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(10.6, 100.0, 15.0),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(
                locations
                    .locations
                    .get("point3d_wall2")
                    .unwrap()
                    .translation,
            ),
        )); // wall1 // wall1
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(10.6, 100.0, 16.0),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(
                locations
                    .locations
                    .get("point3d_wall3")
                    .unwrap()
                    .translation,
            ),
        )); // wall1// wall1
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(3.0, 100.0, 11.7),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(
                locations
                    .locations
                    .get("point3d_wall4")
                    .unwrap()
                    .translation,
            ),
        )); // wall1// wall1
    let mut wall_rot_w4rot = Transform::from_translation(
        locations
            .locations
            .get("point3d_wall4")
            .unwrap()
            .translation,
    );

    wall_rot_w4rot.rotate_local_y(FRAC_PI_4);
    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(10.0, 100.0, 5.0),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(wall_rot_w4rot)); // wall1

    commands
        .spawn()
        .insert_bundle(PhysicsBundle {
            body: RigidBody::Fixed,
            collider: ColliderBundle {
                collider: Collider::cuboid(1.2, 0.58, 2.5),
                groups: ActiveCollisionTypes::all(),
                ..Default::default()
            },
            c_groups: all(),
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(
                locations
                    .locations
                    .get("point3d_scandesk")
                    .unwrap()
                    .translation,
            ),
        )); // wall1// wall1
}

#[derive(SystemParam)]
pub struct OfficeAssetsLookup<'w, 's> {
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub mesh: Res<'w, Assets<Mesh>>,
    pub gltf_mesh: Res<'w, Assets<GltfMesh>>,
    pub gltf_nodes: Res<'w, Assets<GltfNode>>,
    pub gltf: Res<'w, Assets<Gltf>>,
    _macro_appeasment: Query<'w, 's, &'static MacroAppeasementCompnentDONOTUSE>,
}

// NOTE: The SystemParam derive requires 2 lifetimes
// so this gives a bound for our second lifetime
#[derive(Component)]
pub struct MacroAppeasementCompnentDONOTUSE;

pub fn spawn_office(
    mut commands: Commands,
    assets: Res<OfficeAssets>,
    mut lookup: OfficeAssetsLookup,
    mut enities: ResMut<OfficeEntities>,
) {
    let default_material = lookup.materials.add(default());
    for (name, builder) in assets.assets.iter() {
        use OfficeAssetKind::*;
        let entity = match builder.kind {
            Collider => spawn_collider(&mut commands, name, builder, &lookup),
            Sensor => spawn_sensor(&mut commands, name, builder, &lookup),
            Interactable => spawn_interactable(&mut commands, name, builder, &lookup),
            Dynamic => spawn_dynamic(&mut commands, name, builder, &lookup, &default_material),
            Normal => spawn_normal(&mut commands, builder, &lookup, &default_material),
            // note to peng: i moved the Point3D loading somewhere else
            // because it really didn't need to be here
            // so pls no move back :(

            // oh lol
            Point3D | RenderTarget => continue,
            EmissiveNormal => {
                spawn_emissive(&mut commands, builder, &mut lookup);
                continue;
            }
        };
        enities.entities.insert(name, entity);
    }
}

fn spawn_collider(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
) -> Entity {
    let mesh = lookup
        .mesh
        .get(&builder.collider_mesh.clone().unwrap())
        .unwrap();
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(ColliderType::Static)
        .insert(EName {
            id: name.to_string(),
        })
        .insert(group::collide::static_body())
        .insert(ActiveCollisionTypes::all())
        .insert_bundle(TransformBundle::from_transform(builder.trans))
        .insert(static_body())
        .insert(Dominance::group(i8::MAX))
        .id()
}

fn spawn_sensor(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
) -> Entity {
    let mesh = lookup
        .mesh
        .get(&builder.collider_mesh.clone().unwrap())
        .unwrap();
    commands
        .spawn()
        .insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(Sensor)
        .insert(ColliderType::Sensor)
        .insert(sensor())
        .insert(EName {
            id: name.to_string(),
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .insert_bundle(TransformBundle::from_transform(builder.trans))
        .id()
}

fn spawn_interactable(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
) -> Entity {
    let mesh = lookup
        .mesh
        .get(&builder.collider_mesh.clone().unwrap())
        .unwrap();
    commands
        .spawn()
        .insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(ColliderType::Sensor)
        .insert(EName {
            id: name.to_string(),
        })
        .insert(ActiveCollisionTypes::all())
        .insert_bundle(TransformBundle::from_transform(builder.trans))
        .insert(group::collide::interactable_body())
        .insert(Sensor)
        .insert(Interactable::from_name(name))
        .id()
}

fn spawn_dynamic(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
    default_material: &Handle<StandardMaterial>,
) -> Entity {
    // parse the name :skull:
    let name_sections = name.split('_').collect::<Vec<&str>>();
    let friction = name_sections[1].parse::<f32>().unwrap();
    let restitution = name_sections[2].parse::<f32>().unwrap();
    let name = format!("dynamic_{}", name_sections[3]);
    let collider_mesh = lookup
        .mesh
        .get(&builder.collider_mesh.clone().unwrap())
        .unwrap();
    let mesh = lookup.gltf_mesh.get(&builder.mesh).unwrap();
    let collider_material = &mesh.primitives[0].material;

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(Friction::new(friction))
        .insert(Restitution::new(restitution))
        .insert(ColliderType::Dynamic)
        .insert(EName { id: name })
        .insert(ActiveCollisionTypes::all())
        .insert(group::collide::dynamic_body())
        .insert_bundle(TransformBundle::from_transform(builder.trans))
        .insert_bundle(PbrBundle {
            mesh: mesh.primitives[0].mesh.clone(),
            material: collider_material
                .clone()
                .unwrap_or_else(|| default_material.clone()),
            transform: builder.trans,
            ..Default::default()
        })
        .id()
}

fn spawn_normal(
    commands: &mut Commands,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
    default_material: &Handle<StandardMaterial>,
) -> Entity {
    let mesh = lookup.gltf_mesh.get(&builder.mesh).unwrap();
    let mut parent = commands.spawn_bundle(PbrBundle::default());

    parent.with_children(|b| {
        for prim in &mesh.primitives {
            b.spawn_bundle(PbrBundle {
                mesh: prim.mesh.clone(),
                material: prim
                    .material
                    .clone()
                    .unwrap_or_else(|| default_material.clone()),
                transform: builder.trans,
                ..Default::default()
            });
        }
    });

    parent.id()
}

fn spawn_emissive(
    commands: &mut Commands,
    builder: &OfficeAssetBuilder,
    lookup: &mut OfficeAssetsLookup,
) {
    let mesh = lookup.gltf_mesh.get(&builder.mesh).unwrap();
    for prim in &mesh.primitives {
        let current_material =
            unwrap_or_continue!(lookup.materials.get(unwrap_or_continue!(&prim.material)));
        let ecolor = current_material.emissive;
        let etexture = current_material.clone().emissive_texture;
        let new_texture = lookup.materials.add(StandardMaterial {
            emissive: ecolor,
            emissive_texture: etexture,
            ..Default::default()
        });

        // TODO: Just have less fucking lights
        // WARNING: this is hardcoded. only meant for office lights.

        commands
            .spawn()
            .insert_bundle(PointLightBundle {
                point_light: PointLight {
                    intensity: 1000.0,
                    color: Color::rgb(1.0, 0.7, 0.24),
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: builder.trans,
                ..Default::default()
            })
            .insert(LIGHTS_LAYER)
            .with_children(|b| {
                b.spawn_bundle(PbrBundle {
                    mesh: prim.mesh.clone(),
                    material: new_texture,
                    ..Default::default()
                });
            });
    }
}
