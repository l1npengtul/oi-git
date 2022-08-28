use crate::{
    code::{LoCBlock, LocType, LoCEntity},
    collider::{ColliderBundle, PhysicsBundle},
    interactable::{Interactable, InteractableType},
    office::SceneLocations,
    prelude::*,
    unwrap_or_continue,
    utils::EName,
};
use crate::level::NewLevel;
use crate::phys::group::collide::{dynamic_body, interactable_body, interactable_dynamic_body};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, RigidBody, Sensor};

#[derive(AssetCollection)]
pub struct HammerModel {
    #[asset(path = "tools_and_viewmodels/hammer.glb#Scene0")]
    pub hammer_scene: Handle<Scene>,
    #[asset(path = "tools_and_viewmodels/hammer.glb#Animation0")]
    pub swing_animation: Handle<AnimationClip>,
}

pub enum SType {
    Painter,
    Deleter,
    Scanner,
}

pub struct SensorEvent {
    pub stype: SType,
    pub loc: Option<Vec<LoCBlock>>,
}

pub struct ToolsPlugin;

impl ToolsPlugin {
    pub fn spawn_hammer(
        mut commands: Commands,
        locations: Res<SceneLocations>,
        hammer: Res<HammerModel>,
    ) {
        let mut hammer_spawn = *locations.locations.get("point3d_tooldesk").unwrap();
        hammer_spawn.scale = Vec3::ONE;
        hammer_spawn.rotate_local_y(1.57);

        println!("{}", hammer_spawn.translation);
        commands
            .spawn()
            .insert_bundle(SceneBundle {
                scene: hammer.hammer_scene.clone(),
                transform: hammer_spawn,
                ..Default::default()
            })
            .insert_bundle(PhysicsBundle {
                body: RigidBody::Dynamic,
                collider: ColliderBundle {
                    collider: Collider::cuboid(0.08, 0.1, 0.7),
                    ..Default::default()
                },
                c_groups: interactable_dynamic_body(),
                ..Default::default()
            })
            .insert(Interactable::HAMMER)
            .insert(AnimationPlayer::default())
            .insert(LoCEntity);
    }

    // i know, this is pain
    // bear with me
    pub fn loc_sensor_interacts(
        mut events: EventReader<CollisionEvent>,
        mut sensor_event: EventWriter<SensorEvent>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        sensors: Query<&EName, With<Sensor>>,
        mut items: Query<
            (&mut LoCBlock, &Handle<StandardMaterial>),
            (Without<Sensor>, Without<EName>),
        >,
        iteractable: Query<&Interactable, (Without<Sensor>, Without<EName>)>,
        children: Query<&Children>,
    ) {
        for event in events.iter() {
            let (entity1, entity2) = match event {
                CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _) => (e1, e2),
            };

            let ename = match sensors.get(*entity1) {
                Ok(e) => e,
                Err(_) => match sensors.get(*entity2) {
                    Ok(o) => o,
                    Err(_) => continue,
                },
            };

            match ename.id.as_str() {
                "sensor_pp" => {
                    let (mut loc, mat_h) = match items.get_mut(*entity1) {
                        Ok(n) => n,
                        Err(_) => match items.get_mut(*entity2) {
                            Ok(v) => v,
                            Err(_) => continue,
                        },
                    };
                    loc.loc_type = LocType::Green;
                    let mut real_asset = unwrap_or_continue!(materials.get_mut(mat_h));
                    real_asset.base_color = Color::GREEN;
                    sensor_event.send(SensorEvent {
                        stype: SType::Painter,
                        loc: None,
                    });
                }
                "sensor_nn" => {
                    let (mut loc, mat_h) = match items.get_mut(*entity1) {
                        Ok(n) => n,
                        Err(_) => match items.get_mut(*entity2) {
                            Ok(v) => v,
                            Err(_) => continue,
                        },
                    };
                    loc.loc_type = LocType::Red;
                    let mut real_asset = unwrap_or_continue!(materials.get_mut(mat_h));
                    real_asset.base_color = Color::RED;
                    sensor_event.send(SensorEvent {
                        stype: SType::Painter,
                        loc: None,
                    });
                }
                "sensor_ee" => {
                    let (mut loc, mat_h) = match items.get_mut(*entity1) {
                        Ok(n) => n,
                        Err(_) => match items.get_mut(*entity2) {
                            Ok(v) => v,
                            Err(_) => continue,
                        },
                    };
                    loc.loc_type = LocType::Neutral;
                    let mut real_asset = unwrap_or_continue!(materials.get_mut(mat_h));
                    real_asset.base_color = Color::default();
                    sensor_event.send(SensorEvent {
                        stype: SType::Painter,
                        loc: None,
                    });
                }
                "sensor_deleter" => {
                    let (interactable, e) = match iteractable.get(*entity1) {
                        Ok(n) => (n, entity1),
                        Err(_) => match iteractable.get(*entity2) {
                            Ok(v) => (v, entity2),
                            Err(_) => continue,
                        },
                    };
                    match interactable.itype() {
                        InteractableType::LineOfCode | InteractableType::LineOfCodeGlobule => {
                            commands.entity(*e).despawn_recursive();
                            sensor_event.send(SensorEvent {
                                stype: SType::Deleter,
                                loc: None,
                            });
                        }
                        _ => {}
                    }
                }
                "sensor_scandesk" => {
                    let (interactable, e) = match iteractable.get(*entity1) {
                        Ok(n) => (n, entity1),
                        Err(_) => match iteractable.get(*entity2) {
                            Ok(v) => (v, entity2),
                            Err(_) => continue,
                        },
                    };

                    if interactable.itype() == InteractableType::LineOfCode
                        || interactable.itype() == InteractableType::LineOfCodeGlobule
                    {
                        let ch = match children.get(*e) {
                            Ok(c) => c.as_ref().to_vec(),
                            Err(_) => vec![*e],
                        };
                        let mut lines: Vec<LoCBlock> = Vec::with_capacity(ch.len());
                        for line in ch {
                            let (loc, _) = match items.get(line) {
                                Ok(v) => v,
                                Err(_) => continue,
                            };
                            lines.push(loc.clone());
                        }
                        if lines.len() == 2 {
                            lines.reverse();
                        }
                        if !lines.is_empty() {
                            sensor_event.send(SensorEvent {
                                stype: SType::Scanner,
                                loc: Some(lines),
                            })
                        }
                    }
                }
                _ => continue,
            }
        }
    }
    // pub fn manage_loc_sensor_interactions_painters(
    //     rapier: Res<RapierContext>,
    //     mut commands: Commands,
    //     mut materials: ResMut<Assets<StandardMaterial>>,
    //     sensors: Query<(&Collider, &EName, &Transform), With<Sensor>>,
    //     mut items: Query<
    //         (&mut LoCBlock, &Handle<StandardMaterial>),
    //         (Without<Sensor>, Without<EName>),
    //     >,
    // ) {
    //     let query_filter = QueryFilter::exclude_fixed()
    //         .exclude_sensors()
    //         .groups(belong_none_see_interact());
    //     for (collider, s_name, s_transform) in sensors.iter() {
    //         let mut intersecting_objets = vec![];
    //         rapier.intersections_with_shape(
    //             s_transform.translation,
    //             s_transform.rotation,
    //             collider,
    //             query_filter,
    //             |e| {
    //                 intersecting_objets.push(e);
    //                 true
    //             },
    //         );
    //         if !intersecting_objets.is_empty() {
    //             println!("s_name: {}", s_name.id.as_str());
    //         }
    //         match s_name.id.as_str() {
    //             "sensor_deleter" => {
    //                 for iobj in intersecting_objets {
    //                     commands.entity(iobj).despawn_recursive();
    //                 }
    //             }
    //             "sensor_ee" => {
    //                 for iobj in intersecting_objets {
    //                     let (mut loc, mat_h) = unresult_or_continue!(items.get_mut(iobj));
    //                     loc.loc_type = LocType::Neutral;
    //                     let mut real_asset = unwrap_or_continue!(materials.get_mut(mat_h));
    //                     real_asset.base_color = Color::default();
    //                 }
    //             }
    //             "sensor_pp" => {
    //                 for iobj in intersecting_objets {
    //                     let (mut loc, mat_h) = unresult_or_continue!(items.get_mut(iobj));
    //                     loc.loc_type = LocType::Green;
    //                     let mut real_asset = unwrap_or_continue!(materials.get_mut(mat_h));
    //                     real_asset.base_color = Color::GREEN;
    //                 }
    //             }
    //             "sensor_nn" => {
    //                 for iobj in intersecting_objets {
    //                     let (mut loc, mat_h) = unresult_or_continue!(items.get_mut(iobj));
    //                     loc.loc_type = LocType::Red;
    //                     let mut real_asset = unwrap_or_continue!(materials.get_mut(mat_h));
    //                     real_asset.base_color = Color::RED;
    //                 }
    //             }
    //             _ => continue,
    //         }
    //     }
    // }
}

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SensorEvent>();
        app.add_system(ToolsPlugin::loc_sensor_interacts.run_in_state(GameState::InOffice));
        app.add_system(ToolsPlugin::spawn_hammer.run_in_state(GameState::InOffice).run_if(NewLevel::has_triggered));
    }
}
