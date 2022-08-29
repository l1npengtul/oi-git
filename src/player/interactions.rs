use crate::audio::events::{HammerSoundEvent, InteractSoundEvent, InteractSoundType};
use crate::code::LoCEntity;
use crate::{
    collider::{ColliderBundle, PhysicsBundle},
    config::PlayerConfig,
    interactable::{Interactable, InteractableType},
    phys::group::collide::{interactable_dynamic_body, none},
    player::{
        fsm::{PlayerState, PlayerStateMachine},
        PlayerCamera,
    },
    prelude::{phys::*, *},
    viewmodel::{ViewModel, ViewModelHold},
};
use rand::{prelude::SmallRng, Rng, SeedableRng};

pub struct MouseInteraction {
    button: MouseButton,
    with: Entity,
    direction: Vec3,
    pub toi: f32,
}

#[derive(Default)]
pub struct PlayerLookingAt {
    pub entity: Option<Entity>,
    pub dist: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum InteractionSystemLabel {
    Loc,
    LocBundle,
    Hammer,
    HoldingAny,
    HoldingNone,
    RightClick,
    ClearRes,
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct InteractSingleSystemLock {
    pub ran: bool,
}

impl InteractSingleSystemLock {
    // pub fn can_u_run_owo(&self) -> bool {
    //     !self.ran
    // }

    pub fn i_ran_dawddy(&mut self) {
        self.ran = true;
    }
}

pub fn build(app: &mut App) -> &mut App {
    app.add_event::<MouseInteraction>();
    app.insert_resource(InteractSingleSystemLock { ran: false });
    app.init_resource::<PlayerLookingAt>();
    app.add_system(
        MouseInteraction::detect
            .run_in_state(GameState::InOffice)
            .run_unless_resource_equals(PlayerStateMachine::INTERACTING),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingloc_interactwithloctype
            .run_in_state(GameState::InOffice)
            .run_if_resource_equals(InteractSingleSystemLock { ran: false })
            .label(InteractionSystemLabel::Loc)
            .before(InteractionSystemLabel::LocBundle)
            .before(InteractionSystemLabel::Hammer)
            .before(InteractionSystemLabel::HoldingAny)
            .before(InteractionSystemLabel::HoldingNone)
            .before(InteractionSystemLabel::RightClick)
            .before(InteractionSystemLabel::ClearRes),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdinglocbundle_interactwithloctype
            .run_in_state(GameState::InOffice)
            .run_if_resource_equals(InteractSingleSystemLock { ran: false })
            .label(InteractionSystemLabel::LocBundle)
            .after(InteractionSystemLabel::Loc)
            .before(InteractionSystemLabel::Hammer)
            .before(InteractionSystemLabel::HoldingAny)
            .before(InteractionSystemLabel::HoldingNone)
            .before(InteractionSystemLabel::RightClick)
            .before(InteractionSystemLabel::ClearRes),
    );
    // app.add_system(
    //     MouseInteraction::interact_mbleft_holdinghammer_interactwithloc
    //         .run_in_state(GameState::InOffice)
    //         .label(InteractionSystemLabel::Hammer)
    //         .after(InteractionSystemLabel::Loc)
    //         .before(InteractionSystemLabel::Hammer)
    //         .before(InteractionSystemLabel::HoldingAny)
    //         .before(InteractionSystemLabel::HoldingNone),
    // );
    // app.add_system(
    //     MouseInteraction::interact_mbleft_holdinghammer_interactwithlocbundle
    //         .run_in_state(GameState::InOffice),
    // );
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::InOffice)
            .run_if_resource_equals(InteractSingleSystemLock { ran: false })
            .label(InteractionSystemLabel::Hammer)
            .after(InteractionSystemLabel::Loc)
            .after(InteractionSystemLabel::LocBundle)
            .before(InteractionSystemLabel::HoldingAny)
            .before(InteractionSystemLabel::HoldingNone)
            .before(InteractionSystemLabel::ClearRes)
            .before(InteractionSystemLabel::RightClick)
            .with_system(MouseInteraction::interact_mbleft_holdinghammer_interactwithloc)
            .with_system(MouseInteraction::interact_mbleft_holdinghammer_interactwithlocbundle)
            .with_system(MouseInteraction::interact_mbleft_holdinghammer_interactnone)
            .into(),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingany_interactterminal
            .run_in_state(GameState::InOffice)
            .run_if_resource_equals(InteractSingleSystemLock { ran: false })
            .label(InteractionSystemLabel::HoldingAny)
            .after(InteractionSystemLabel::Loc)
            .after(InteractionSystemLabel::LocBundle)
            .after(InteractionSystemLabel::Hammer)
            .before(InteractionSystemLabel::HoldingNone)
            .before(InteractionSystemLabel::RightClick)
            .before(InteractionSystemLabel::ClearRes),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingnone_interactany
            .run_in_state(GameState::InOffice)
            .run_if_resource_equals(InteractSingleSystemLock { ran: false })
            .label(InteractionSystemLabel::HoldingNone)
            .after(InteractionSystemLabel::Loc)
            .after(InteractionSystemLabel::LocBundle)
            .after(InteractionSystemLabel::Hammer)
            .after(InteractionSystemLabel::HoldingAny)
            .before(InteractionSystemLabel::RightClick)
            .before(InteractionSystemLabel::ClearRes),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingnone_interactany
            .run_in_state(GameState::InOffice)
            .run_if_resource_equals(InteractSingleSystemLock { ran: false })
            .label(InteractionSystemLabel::HoldingNone)
            .after(InteractionSystemLabel::Loc)
            .after(InteractionSystemLabel::LocBundle)
            .after(InteractionSystemLabel::Hammer)
            .after(InteractionSystemLabel::HoldingAny)
            .before(InteractionSystemLabel::ClearRes),
    );
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::InOffice)
            .label(InteractionSystemLabel::RightClick)
            .after(InteractionSystemLabel::Loc)
            .after(InteractionSystemLabel::LocBundle)
            .after(InteractionSystemLabel::Hammer)
            .after(InteractionSystemLabel::HoldingAny)
            .after(InteractionSystemLabel::HoldingNone)
            .before(InteractionSystemLabel::ClearRes)
            .with_system(MouseInteraction::interact_mbright_holdingany_interactnone)
            .with_system(MouseInteraction::interact_mbright_holdingany_interactany)
            .into(),
    );
    app.add_system(
        MouseInteraction::clear_reader_because_fuck_you
            .run_in_state(GameState::InOffice)
            .label(InteractionSystemLabel::ClearRes)
            .after(InteractionSystemLabel::Loc)
            .after(InteractionSystemLabel::LocBundle)
            .after(InteractionSystemLabel::Hammer)
            .after(InteractionSystemLabel::HoldingAny)
            .after(InteractionSystemLabel::HoldingNone)
            .after(InteractionSystemLabel::RightClick),
    );

    app
}

#[derive(Component, Clone, Debug)]
pub struct OrderedChildren(pub Vec<Entity>);

impl MouseInteraction {
    pub fn detect(
        mut lock: ResMut<InteractSingleSystemLock>,
        player_config: Res<PlayerConfig>,
        mut interacts: EventWriter<MouseInteraction>,
        bttns: Res<Input<MouseButton>>,
        rapier: Res<RapierContext>,
        camera_query: Query<&Transform, With<PlayerCamera>>,
        mut looking_at: ResMut<PlayerLookingAt>,
    ) {
        let camera_trans = camera_query.single();
        let mut pressed = bttns.get_just_pressed();

        // lmb has been pressed
        let ray_origin = camera_trans.translation;
        let ray_dir = camera_trans.rotation * -Vec3::Z;
        let max_toi = player_config.reach_dist;
        let solid = false;
        let groups = group::interact::player_vision();
        let filter = groups.into();
        //
        // let viewmodel_children = world.get::<Children>(vm_ent).unwrap();
        // let vm_child_id = viewmodel_children[0];
        //
        if let Some((entity, toi)) = rapier.cast_ray(ray_origin, ray_dir, max_toi, solid, filter) {
            if toi < max_toi {
                *looking_at = PlayerLookingAt {
                    entity: Some(entity),
                    dist: toi,
                };
                if let Some(button) = pressed.next() {
                    lock.ran = false;
                    interacts.send(MouseInteraction {
                        button: *button,
                        with: entity,
                        direction: ray_dir,
                        toi,
                    });
                }
            }
        } else {
            looking_at.entity = None
        }
    }

    pub fn interact_mbleft_holdingloc_interactwithloctype(
        mut commands: Commands,
        mut reader: EventReader<MouseInteraction>,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
        mut lock: ResMut<InteractSingleSystemLock>,
        mut player_state: ResMut<PlayerStateMachine>,
        children: Query<&Children>,
        mut ordered_children: Query<&mut OrderedChildren>,
        mut transform: Query<&mut Transform>,
        mut viewmodel_query: Query<(&mut ViewModel, Entity), With<ViewModel>>,
        interact_type: Query<&Interactable>,
    ) {
        let (mut viewmodel, vm_ent) = viewmodel_query.single_mut();
        let vm_child_id = match children.get(vm_ent) {
            Ok(v) => match v.get(0) {
                Some(v) => *v,
                None => return,
            },
            Err(_) => return,
        };
        for event in reader.iter() {
            let interacting_ent = event.with;
            let interact_typ = match interact_type.get(interacting_ent) {
                Ok(inter) => *inter,
                Err(_) => {
                    continue;
                }
            };
            if event.button == MouseButton::Left
                && viewmodel.holding() == ViewModelHold::LoC
                && interact_typ == Interactable::LOC
            {
                let current_gnd_trans = transform.get_mut(interacting_ent).unwrap();
                let parent_new_trans = *current_gnd_trans;
                let new = commands
                    .spawn()
                    .insert_bundle(TransformBundle::from_transform(parent_new_trans))
                    .insert(Interactable::LOCG)
                    .insert_bundle(PhysicsBundle {
                        body: RigidBody::Dynamic,
                        collider: ColliderBundle {
                            collider: Collider::cuboid(0.1, 0.015, 0.75),
                            groups: ActiveCollisionTypes::all(),
                            ..Default::default()
                        },
                        c_groups: interactable_dynamic_body(),
                        ..Default::default()
                    })
                    .insert(OrderedChildren(vec![interacting_ent, vm_child_id]))
                    .insert(LoCEntity)
                    .id();
                commands
                    .entity(vm_child_id)
                    .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                        -0.05, 0.0, 0.0,
                    )))
                    .insert(RigidBody::Fixed)
                    .insert(none())
                    .insert(ActiveCollisionTypes::empty())
                    .remove::<Interactable>();
                commands
                    .entity(interacting_ent)
                    .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                        0.05, 0.0, 0.0,
                    )))
                    .insert(RigidBody::Fixed)
                    .insert(none())
                    .insert(ActiveCollisionTypes::empty())
                    .remove::<Interactable>();
                commands
                    .entity(vm_ent)
                    .remove_children(children.get(vm_ent).unwrap());
                commands
                    .entity(new)
                    .push_children(&[interacting_ent, vm_child_id]);
                viewmodel.change_holding(ViewModelHold::Empty);
                player_state.change_state(PlayerState::Idle);
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::Attach,
                });
                lock.i_ran_dawddy();
                return;
            } else if event.button == MouseButton::Left
                && viewmodel.holding() == ViewModelHold::LoC
                && interact_typ == Interactable::LOCG
            {
                let mut existing_children = ordered_children.get_mut(interacting_ent).unwrap();

                commands
                    .entity(vm_ent)
                    .remove_children(children.get(vm_ent).unwrap());
                commands
                    .entity(interacting_ent)
                    .remove_children(&existing_children.0);
                let children_offset = existing_children.0.len() as i32 / 2;
                existing_children.0.push(vm_child_id);
                dbg!(&*existing_children);
                for (i, &child) in existing_children.0.iter().enumerate() {
                    let new_item_trans = Transform::from_xyz(
                        0.1 * (i as i32 - children_offset) as f32 * -1.0,
                        0.0,
                        0.0,
                    );
                    // commands.entity(child).log_components();
                    commands
                        .entity(child)
                        .insert(new_item_trans)
                        .insert(RigidBody::Fixed)
                        .insert(none())
                        .insert(ActiveCollisionTypes::empty());

                    commands.entity(interacting_ent).add_child(child);
                }
                viewmodel.change_holding(ViewModelHold::Empty);

                let hx = {
                    let ch_len = existing_children.0.len() + 1;
                    if existing_children.0.len() % 2 == 0 {
                        0.05 * ch_len as f32
                    } else {
                        0.05 * (ch_len - 1) as f32 + 0.025
                    }
                };

                commands
                    .entity(interacting_ent)
                    .insert(Collider::cuboid(hx, 0.015, 0.75))
                    .insert(LoCEntity);
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::Attach,
                });
                player_state.change_state(PlayerState::Idle);
                lock.i_ran_dawddy();
                return;
            }
        }
    }

    pub fn interact_mbleft_holdinglocbundle_interactwithloctype(
        mut lock: ResMut<InteractSingleSystemLock>,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
        mut reader: EventReader<MouseInteraction>,
        mut commands: Commands,
        mut player_state: ResMut<PlayerStateMachine>,
        transform: Query<&Transform>,
        mut viewmodel_query: Query<(&mut ViewModel, Entity, &Children), With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        children: Query<&Children>,
        mut ordered_children: Query<&mut OrderedChildren>,
    ) {
        let (mut viewmodel, vm_ent, vm_children) = viewmodel_query.single_mut();
        let vm_child_id: Entity = match vm_children.get(0) {
            Some(v) => *v,
            None => return,
        };

        for event in reader.iter() {
            let interacting_ent = event.with;
            let interact_typ = match interact_type.get(interacting_ent) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };

            if event.button == MouseButton::Left && viewmodel.holding() == ViewModelHold::LoCBundle
            {
                let new_locg_position = *transform.get(interacting_ent).unwrap();
                let mut interact_to_add = match interact_typ.itype() {
                    InteractableType::LineOfCode => {
                        vec![interacting_ent]
                    }
                    InteractableType::LineOfCodeGlobule => {
                        // event,with is an LOCG
                        let a = &mut ordered_children.get_mut(interacting_ent).unwrap().0;

                        commands.entity(interacting_ent).remove_children(&a);
                        commands.entity(interacting_ent).despawn(); // despawn the LOCG itself
                        let ret = a.clone();
                        a.clear();
                        ret
                    }
                    _ => return,
                };
                // now append our thing
                let mut new_locg_things = &mut ordered_children.get_mut(vm_child_id).unwrap().0;
                interact_to_add.append(&mut new_locg_things);
                let new_locg_things = interact_to_add;
                // now for some cleanup
                commands
                    .entity(vm_ent)
                    .remove_children(children.get(vm_ent).unwrap());
                commands
                    .entity(vm_child_id)
                    .remove_children(&new_locg_things);
                // create the new locg

                let new_locg = commands
                    .spawn()
                    .insert_bundle(TransformBundle::from_transform(new_locg_position))
                    .insert(Interactable::LOCG)
                    .insert_bundle(PhysicsBundle {
                        body: RigidBody::Dynamic,
                        collider: ColliderBundle {
                            collider: Collider::cuboid(0.1, 0.015, 0.75),
                            groups: ActiveCollisionTypes::all(),
                            ..Default::default()
                        },
                        c_groups: interactable_dynamic_body(),
                        ..Default::default()
                    })
                    .insert(OrderedChildren(new_locg_things.clone()))
                    .insert(LoCEntity)
                    .id();
                // insert the childrernn
                let children_offset = new_locg_things.len() as i32 / 2;
                for (i, child) in new_locg_things.iter().enumerate() {
                    let new_item_trans = Transform::from_xyz(
                        0.1 * (i as i32 - children_offset) as f32 * -1.0,
                        0.0,
                        0.0,
                    );
                    commands
                        .entity(*child)
                        .insert(new_item_trans)
                        .insert(RigidBody::Fixed)
                        .insert(none())
                        .insert(ActiveCollisionTypes::empty())
                        .insert(Collider::cuboid(0.05, 0.015, 0.75));
                    commands.entity(new_locg).push_children(&[*child]);
                }
                player_state.change_state(PlayerState::Idle);
                viewmodel.change_holding(ViewModelHold::Empty);

                let hx = {
                    let ch_len = new_locg_things.len();
                    if new_locg_things.len() % 2 == 0 {
                        0.05 * ch_len as f32
                    } else {
                        0.05 * (ch_len - 1) as f32 + 0.05
                    }
                };

                commands
                    .entity(new_locg)
                    .insert(Collider::cuboid(hx, 0.015, 0.75));
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::Attach,
                });
                lock.i_ran_dawddy();
                return;
            }
        }
    }

    pub fn interact_mbleft_holdinghammer_interactwithloc(
        mut lock: ResMut<InteractSingleSystemLock>,
        mut reader: EventReader<MouseInteraction>,
        mut sound_event: EventWriter<HammerSoundEvent>,
        mut commands: Commands,
        mut viewmodel_query: Query<&ViewModel>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
    ) {
        let viewmodel = viewmodel_query.single_mut();

        for event in reader.iter() {
            let interact_typ = match interact_type.get(event.with) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };
            if event.button == MouseButton::Left
                && viewmodel.holding() == ViewModelHold::Hammer
                && interact_typ == Interactable::LOC
            {
                let ray_dir = event.direction;
                let ray_dir_y_inv = Vec3::new(ray_dir.x, -ray_dir.y, ray_dir.z);
                commands.entity(event.with).insert(ExternalImpulse {
                    impulse: ray_dir_y_inv * 0.05,
                    ..Default::default()
                });
                sound_event.send(HammerSoundEvent { hit: true });
                lock.i_ran_dawddy();
            }
        }
    }

    pub fn interact_mbleft_holdinghammer_interactwithlocbundle(
        mut lock: ResMut<InteractSingleSystemLock>,
        mut reader: EventReader<MouseInteraction>,
        mut sound_event: EventWriter<HammerSoundEvent>,
        mut commands: Commands,
        mut viewmodel_query: Query<&mut ViewModel, With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        transgender: Query<&Transform>,
        children: Query<&Children>,
    ) {
        let viewmodel = viewmodel_query.single_mut();

        for event in reader.iter() {
            if event.toi > 1.5 {
                continue;
            }
            let interact_typ = match interact_type.get(event.with) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };
            if event.button == MouseButton::Left
                && viewmodel.holding() == ViewModelHold::Hammer
                && interact_typ == Interactable::LOCG
            {
                let floor_item_children = match children.get(event.with) {
                    Ok(c) => c,
                    Err(_) => return,
                };

                let current_bundle_trans = *transgender.get(event.with).unwrap();
                let mut random = SmallRng::from_entropy();
                for (i, e) in floor_item_children.iter().enumerate() {
                    commands.entity(event.with).remove_children(&[*e]);
                    let random_x = random.gen_range(-1.0..=1.0);
                    let random_z = random.gen_range(-1.0..=1.0);
                    let force = Vec3::new(random_x, 1.0, random_z);
                    let mut new_trans = current_bundle_trans;
                    new_trans.translation.y += 0.02 * i as f32;
                    commands
                        .entity(*e)
                        .insert_bundle(TransformBundle::from_transform(new_trans))
                        .insert(ExternalImpulse {
                            impulse: force * 0.015,
                            ..Default::default()
                        })
                        .insert(Interactable::LOC)
                        .insert(RigidBody::Dynamic)
                        .insert(ActiveCollisionTypes::all())
                        .insert(interactable_dynamic_body());
                }
                commands.entity(event.with).despawn();
                sound_event.send(HammerSoundEvent { hit: true });
                lock.i_ran_dawddy();
            }
        }
    }

    pub fn interact_mbleft_holdinghammer_interactnone(
        mut events: EventWriter<HammerSoundEvent>,
        bttns: Res<Input<MouseButton>>,
        looking_at: Res<PlayerLookingAt>,
        viewmodel_query: Query<&ViewModel, With<ViewModel>>,
        interactable_q: Query<&Interactable>,
    ) {
        let viewmodel = match viewmodel_query.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };

        for event in bttns.get_just_pressed() {
            let interacttyp = looking_at.entity.and_then(|e| interactable_q.get(e).ok());

            if *event == MouseButton::Left
                && interacttyp.is_none()
                && viewmodel.holding() == ViewModelHold::Hammer
            {
                events.send(HammerSoundEvent { hit: false });
            }
        }
    }

    pub fn interact_mbleft_holdingany_interactterminal(
        mut lock: ResMut<InteractSingleSystemLock>,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
        mut reader: EventReader<MouseInteraction>,
        mut state: ResMut<PlayerStateMachine>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
    ) {
        for event in reader.iter() {
            let interact_typ = match interact_type.get(event.with) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };
            if event.button == MouseButton::Left && interact_typ == Interactable::TERMINAL {
                state.change_state(PlayerState::Interacting);
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::TerminalEnter,
                });
                lock.i_ran_dawddy();
            }
        }
    }

    pub fn interact_mbleft_holdingnone_interactany(
        mut lock: ResMut<InteractSingleSystemLock>,
        mut reader: EventReader<MouseInteraction>,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
        mut commands: Commands,
        mut viewmodel_query: Query<(&mut ViewModel, Entity), With<ViewModel>>,
        mut player_state: ResMut<PlayerStateMachine>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
    ) {
        let (mut viewmodel, vm_ent) = match viewmodel_query.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };
        // remove the first entity if it has nothing but parent

        for event in reader.iter() {
            let interacting_ent = event.with;
            let interact_typ = match interact_type.get(interacting_ent) {
                Ok(inter) => *inter,
                Err(_) => return,
            };
            if event.button == MouseButton::Left && viewmodel.holding() == ViewModelHold::Empty {
                let new_view_type = match interact_typ.itype() {
                    InteractableType::Hammer => ViewModelHold::Hammer,
                    InteractableType::LineOfCode => ViewModelHold::LoC,
                    InteractableType::LineOfCodeGlobule => ViewModelHold::LoCBundle,
                    InteractableType::Terminal => return,
                };

                let transgender = match new_view_type {
                    ViewModelHold::Empty => return,
                    ViewModelHold::Hammer => {
                        let mut new_trans = Transform::from_xyz(-0.2, 0.3, 0.0);
                        new_trans.rotate_local_y(-1.57);
                        new_trans.rotate_local_x(0.3491);
                        new_trans.rotate_local_z(-0.1745329);
                        new_trans
                    }
                    ViewModelHold::LoC => {
                        let mut new_trans = Transform::from_xyz(-0.2, 0.5, 0.0);
                        new_trans.rotate_local_y(1.57);
                        new_trans.rotate_local_x(-0.1745329);
                        new_trans.rotate_local_z(0.3491);
                        new_trans
                    }
                    ViewModelHold::LoCBundle => {
                        let mut new_trans = Transform::from_xyz(-0.2, 0.3, 0.0);
                        new_trans.rotate_local_y(-1.57);
                        new_trans.rotate_local_x(2.967);
                        new_trans.rotate_local_z(-1.);
                        new_trans
                    }
                };
                viewmodel.change_holding(new_view_type);
                commands
                    .entity(interacting_ent)
                    .remove::<Interactable>()
                    .remove::<RigidBody>()
                    .remove::<ActiveCollisionTypes>()
                    .remove::<CollisionGroups>()
                    .remove::<Transform>()
                    .insert(transgender)
                    .insert(ActiveCollisionTypes::empty())
                    .insert(none())
                    .insert(RigidBody::Fixed);
                commands.entity(vm_ent).despawn_descendants();
                commands.entity(vm_ent).push_children(&[interacting_ent]);
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::Pickup,
                });
                player_state.change_state(PlayerState::Holding);
                lock.i_ran_dawddy();
                return;
            }
        }
    }

    pub fn interact_mbright_holdingany_interactnone(
        mut commands: Commands,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
        looking_at: Res<PlayerLookingAt>,
        mut player_state: ResMut<PlayerStateMachine>,
        bttns: Res<Input<MouseButton>>,
        mut viewmodel_query: Query<(&mut ViewModel, Entity, &Children), With<ViewModel>>,
        camera_query: Query<&Transform, With<PlayerCamera>>,
        player_query: Query<&Velocity, With<Player>>,
        interactable_q: Query<&Interactable>,
        children: Query<&Children>,
    ) {
        let (mut viewmodel, vm_ent, vm_children) = match viewmodel_query.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };
        let camera_trans = camera_query.single();
        let player_vel = player_query.get_single().copied().unwrap_or_default();
        // remove the first entity if it has nothing but parent

        for event in bttns.get_just_pressed() {
            let interacttyp = looking_at.entity.and_then(|e| interactable_q.get(e).ok());
            if *event == MouseButton::Right && interacttyp.is_none() {
                let force_dir = camera_trans.rotation * -Vec3::Z;

                // new tranform
                let vm_trans = Vec3::new(0.0, 0.0, -1.0);
                let c_rot = camera_trans.rotation;
                let fin = ((c_rot * vm_trans).normalize_or_zero() * 2.0) + camera_trans.translation;

                let child: Entity = match vm_children.get(0) {
                    Some(v) => *v,
                    None => return,
                };

                let interact_type = match viewmodel.holding() {
                    ViewModelHold::Empty => return,
                    ViewModelHold::Hammer => Interactable::HAMMER,
                    ViewModelHold::LoCBundle => Interactable::LOCG,
                    ViewModelHold::LoC => Interactable::LOC,
                };

                let force_extra_multi = match interact_type.itype() {
                    InteractableType::Hammer => 4.0,
                    InteractableType::LineOfCode => 1.0,
                    InteractableType::LineOfCodeGlobule => match children.get(child) {
                        Ok(c) => c.len() as f32,
                        Err(_) => 1.0,
                    },
                    InteractableType::Terminal => return,
                };

                commands.entity(vm_ent).remove_children(&[child]);
                commands
                    .entity(child)
                    .insert_bundle(TransformBundle::from_transform(
                        Transform::from_translation(fin),
                    ))
                    .insert(RigidBody::Dynamic)
                    .insert(ActiveCollisionTypes::all())
                    .insert(interactable_dynamic_body())
                    .insert(ExternalImpulse {
                        impulse: force_dir * 0.05 * force_extra_multi,
                        ..Default::default()
                    })
                    .insert(interact_type)
                    .insert(player_vel);

                viewmodel.change_holding(ViewModelHold::Empty);
                player_state.change_state(PlayerState::Idle);
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::Throw,
                });
            }
        }
    }

    pub fn interact_mbright_holdingany_interactany(
        mut lock: ResMut<InteractSingleSystemLock>,
        mut reader: EventReader<MouseInteraction>,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
        mut commands: Commands,
        mut viewmodel_query: Query<(&mut ViewModel, Entity, &Children), With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        transforms: Query<&Transform>,
    ) {
        let (mut viewmodel, vm_ent, vm_children) = match viewmodel_query.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };
        // remove the first entity if it has nothing but parent

        for event in reader.iter() {
            let interacting_ent = event.with;
            let interact_typ = match interact_type.get(interacting_ent) {
                Ok(inter) => *inter,
                Err(_) => return,
            };
            if event.button == MouseButton::Right
                && viewmodel.holding() != ViewModelHold::Empty
                && interact_typ != Interactable::TERMINAL
            {
                let ground_item_position = transforms.get(interacting_ent).unwrap();
                let viewmodel_held: Entity = *vm_children.get(0).unwrap();

                let new_view_type = match interact_typ.itype() {
                    InteractableType::Hammer => ViewModelHold::Hammer,
                    InteractableType::LineOfCode => ViewModelHold::LoC,
                    InteractableType::LineOfCodeGlobule => ViewModelHold::LoCBundle,
                    InteractableType::Terminal => return,
                };

                let interact_type = match viewmodel.holding() {
                    ViewModelHold::Empty => return,
                    ViewModelHold::Hammer => Interactable::HAMMER,
                    ViewModelHold::LoCBundle => Interactable::LOCG,
                    ViewModelHold::LoC => Interactable::LOC,
                };

                let transgender = match new_view_type {
                    ViewModelHold::Empty => return,
                    ViewModelHold::Hammer => {
                        let mut new_trans = Transform::from_xyz(-0.2, 0.3, 0.0);
                        new_trans.rotate_local_y(-1.57);
                        new_trans.rotate_local_x(0.3491);
                        new_trans.rotate_local_z(-0.1745329);
                        new_trans
                    }
                    ViewModelHold::LoC => {
                        let mut new_trans = Transform::from_xyz(-0.2, 0.5, 0.0);
                        new_trans.rotate_local_y(1.57);
                        new_trans.rotate_local_x(-0.1745329);
                        new_trans.rotate_local_z(0.3491);
                        new_trans
                    }
                    ViewModelHold::LoCBundle => {
                        let mut new_trans = Transform::from_xyz(-0.2, 0.3, 0.0);
                        new_trans.rotate_local_y(-1.57);
                        new_trans.rotate_local_x(2.967);
                        new_trans.rotate_local_z(-1.);
                        new_trans
                    }
                };
                viewmodel.change_holding(new_view_type);

                commands.entity(vm_ent).remove_children(&[viewmodel_held]);
                commands
                    .entity(viewmodel_held)
                    .insert_bundle(TransformBundle::from_transform(*ground_item_position))
                    .insert(RigidBody::Dynamic)
                    .insert(ActiveCollisionTypes::all())
                    .insert(interactable_dynamic_body())
                    .insert(interact_type);
                let new_hold = commands
                    .entity(interacting_ent)
                    .remove::<Interactable>()
                    .remove::<RigidBody>()
                    .remove::<ActiveCollisionTypes>()
                    .remove::<CollisionGroups>()
                    .remove::<Transform>()
                    .insert(transgender)
                    .insert(ActiveCollisionTypes::empty())
                    .insert(none())
                    .insert(RigidBody::Fixed)
                    .id();
                commands.entity(vm_ent).push_children(&[new_hold]);
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::Pickup,
                });
                lock.i_ran_dawddy();
            }
        }
    }
    pub fn clear_reader_because_fuck_you(reader: EventReader<MouseInteraction>) {
        reader.clear()
    }
    // throw if right click
    //     // if left click: swing hammer, or drop
    //     match pressed.peek().unwrap() {
    //         // safe since we know one has been pressed at least
    //         // S W I N G
    //         MouseButton::Left => {
    //             match viewmodel.holding() {
    //                 ViewModelHold::Hammer => {
    //                     // swing
    //                 }
    //                 ViewModelHold::Empty => (),
    //                 _ => { //drop
    //                 }
    //             }
    //         }
    //         // Y E E T held item
    //         MouseButton::Right => {
    //             println!("YEET (TODO!)");
    //             viewmodel.change_holding(ViewModelHold::Empty);
    //             commands.entity(vm_ent).remove_children(&[vm_child_id]);
    //         }
    //         _ => (),
    //     }

    // TODO
    // pub fn interact_mbleft_holding_hammer() {}
    // pub fn interact_mbleft_holding_nonhammer() {}
    // pub fn interact_mbright() {}
}
