use super::{OfficeAssetBuilder, OfficeAssetKind, OfficeAssets};
use crate::prelude::{phys::*, utils::*, *};
use bevy::ecs::system::SystemParam;
use bevy::gltf::{Gltf, GltfMesh, GltfNode};

#[derive(SystemParam)]
pub struct OfficeAssetsLookup<'w, 's> {
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    // pub scene_locations: ResMut<'w, SceneLocations>,
    // pub target: Res<'w, TerminalScreenTarget>,
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
) {
    let default_material = lookup.materials.add(default());
    for (name, builder) in assets.assets.iter() {
        use OfficeAssetKind::*;
        match builder.kind {
            Collider => spawn_collider(&mut commands, name, builder, &lookup),
            Sensor => spawn_sensor(&mut commands, name, builder, &lookup),
            Dynamic => spawn_dynamic(&mut commands, name, builder, &lookup, &default_material),
            Normal => spawn_normal(&mut commands, builder, &lookup, &default_material),
            Point3D | RenderTarget => (),
            EmissiveNormal => spawn_emissive(&mut commands, builder, &mut lookup)
        }
    }
}

fn spawn_collider(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
) {
    let mesh = lookup
        .mesh
        .get(&builder.collider_mesh.clone().unwrap())
        .unwrap();
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(ColliderType::Static)
        .insert(EName { id: name.to_string() })
        .insert_bundle(TransformBundle::from_transform(builder.trans));
}

fn spawn_sensor(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
) {
    let mesh = lookup
        .mesh
        .get(&builder.collider_mesh.clone().unwrap())
        .unwrap();
    commands
        .spawn()
        .insert(Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(Sensor)
        .insert(ColliderType::Sensor)
        .insert(EName { id: name.to_string() })
        .insert_bundle(TransformBundle::from_transform(builder.trans));
}

fn spawn_dynamic(
    commands: &mut Commands,
    name: &str,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
    default_material: &Handle<StandardMaterial>,
) {
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
        .insert_bundle(TransformBundle::from_transform(builder.trans))
        .insert_bundle(PbrBundle {
            mesh: mesh.primitives[0].mesh.clone(),
            material: collider_material
                .clone()
                .unwrap_or_else(|| default_material.clone()),
            transform: builder.trans,
            ..Default::default()
        });
}

fn spawn_normal(
    commands: &mut Commands,
    builder: &OfficeAssetBuilder,
    lookup: &OfficeAssetsLookup,
    default_material: &Handle<StandardMaterial>,
) {
    let mesh = lookup.gltf_mesh.get(&builder.mesh).unwrap();
    for prim in &mesh.primitives {
        commands.spawn_bundle(PbrBundle {
            mesh: prim.mesh.clone(),
            material: prim
                .material
                .clone()
                .unwrap_or_else(|| default_material.clone()),
            transform: builder.trans,
            ..Default::default()
        });
    }
}

fn spawn_emissive(
    commands: &mut Commands,
    builder: &OfficeAssetBuilder,
    lookup: &mut OfficeAssetsLookup) {
    let mesh = lookup.gltf_mesh.get(&builder.mesh).unwrap();
    for prim in &mesh.primitives {
        let current_material = unwrap_or_continue!( lookup.materials.get(unwrap_or_continue!(&prim.material)));
        let ecolor = current_material.emissive;
        let etexture = current_material.clone().emissive_texture;
        let new_texture = lookup.materials.add(StandardMaterial {
            emissive: ecolor,
            emissive_texture: etexture,
            ..Default::default()
        });

        commands.spawn_bundle(PbrBundle {
            mesh: prim.mesh.clone(),
            material: new_texture,
            transform: builder.trans,
            ..Default::default()
        });
    }
}
