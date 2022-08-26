use super::{OfficeAssetBuilder, OfficeAssetKind, OfficeAssets, OfficeEntities};
use crate::interactable::Interactable;
use crate::phys::group::collide::player_vision;
use crate::phys::group::collide::static_body;
use crate::prelude::{phys::*, utils::*, *};
use bevy::ecs::system::SystemParam;
use bevy::gltf::{Gltf, GltfMesh, GltfNode};

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
        .insert(EName {
            id: name.to_string(),
        })
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
        .insert(player_vision())
        .insert(Sensor)
        .insert(Interactable::TERMINAL) // LOL
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
                    intensity: 1600.0,
                    color: Color::rgb(1.0, 0.65, 0.24),
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
