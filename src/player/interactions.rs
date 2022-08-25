use crate::{
    config::PlayerConfig,
    interactable::{Interactable, InteractableType},
    player::{
        fsm::{PlayerState, PlayerStateMachine},
        PlayerCamera,
    },
    prelude::{phys::*, *},
    viewmodel::{ViewModel, ViewModelHold},
};

use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

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

pub fn build(app: &mut App) {
    app.add_event::<MouseInteraction>();
    app.add_system(
        MouseInteraction::detect
            .run_in_state(GameState::MainMenu)
            .run_unless_resource_equals(PlayerStateMachine::INTERACTING),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingloc_interactwithloctype
            .run_in_state(GameState::MainMenu),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdinglocbundle_interactwithloctype
            .run_in_state(GameState::MainMenu),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdinghammer_interactwithloc
            .run_in_state(GameState::MainMenu),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdinghammer_interactwithlocbundle
            .run_in_state(GameState::MainMenu),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingany_interactterminal
            .run_in_state(GameState::MainMenu),
    );
    app.add_system(
        MouseInteraction::interact_mbleft_holdingnone_interactany.run_in_state(GameState::MainMenu),
    );
    app.init_resource::<PlayerLookingAt>();
}

impl MouseInteraction {
    pub fn detect(
        player_config: Res<PlayerConfig>,
        mut interacts: EventWriter<MouseInteraction>,
        bttns: Res<Input<MouseButton>>,
        rapier: Res<RapierContext>,
        camera_query: Query<&Transform, With<PlayerCamera>>,
        mut looking_at: ResMut<PlayerLookingAt>,
    ) {
        let camera_trans = camera_query.single();
        let mut pressed = bttns.get_just_pressed().peekable();
        if pressed.len() == 0 {
            return;
        }
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
            *looking_at = PlayerLookingAt {
                entity: Some(entity),
                dist: toi,
            };
            interacts.send(MouseInteraction {
                button: **pressed.peek().unwrap(),
                with: entity,
                direction: ray_dir,
                toi,
            })
        } else {
            looking_at.entity = None
        }
    }

    pub fn interact_mbleft_holdingloc_interactwithloctype(
        mut reader: EventReader<MouseInteraction>,
        mut commands: Commands,
        mut state: ResMut<PlayerStateMachine>,
        mut viewmodel_query: Query<(&mut ViewModel, Entity, &Children), With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        children: Query<&Children, (Without<Interactable>, Without<ViewModel>)>,
    ) {
        let (mut viewmodel, vm_ent, vm_children) = viewmodel_query.single_mut();
        let vm_child_id: Entity = match vm_children.get(0) {
            Some(v) => *v,
            None => {
                let new_ent = commands.spawn().id();
                commands.entity(vm_ent).add_child(new_ent);
                new_ent
            }
        };
        for event in reader.iter() {
            let interact_typ = match interact_type.get(event.with) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };
            if event.button == MouseButton::Left
                && viewmodel.holding() == ViewModelHold::LoC
                && interact_typ == Interactable::LOC
                && interact_typ == Interactable::LOCG
            {
                let floor_item_children = match children.get(event.with) {
                    Ok(c) => c,
                    Err(_) => {
                        commands.entity(event.with).add_children(|_| {});
                        children.get(event.with).unwrap()
                    }
                };
                viewmodel.change_holding(ViewModelHold::Empty);
                state.change_state(PlayerState::Idle);
                // remove the viewmodel child
                commands.entity(vm_ent).remove_children(&[vm_child_id]);
                commands
                    .entity(vm_child_id)
                    .insert(ActiveCollisionTypes::default())
                    .insert(Transform::from_xyz(
                        // FIXME: Adjust
                        0.0,
                        0.0,
                        -0.3 * (floor_item_children.len() + 1) as f32,
                    ));
                commands.entity(event.with).push_children(&[vm_child_id]);
            }
        }
    }

    pub fn interact_mbleft_holdinglocbundle_interactwithloctype(
        mut reader: EventReader<MouseInteraction>,
        mut commands: Commands,
        mut state: ResMut<PlayerStateMachine>,
        mut viewmodel_query: Query<(&mut ViewModel, Entity, &Children), With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        children: Query<&Children, (Without<Interactable>, Without<ViewModel>)>,
    ) {
        let (mut viewmodel, vm_ent, vm_children) = viewmodel_query.single_mut();
        let vm_child_id: Entity = match vm_children.get(0) {
            Some(v) => *v,
            None => {
                let new_ent = commands.spawn().id();
                commands.entity(vm_ent).add_child(new_ent);
                new_ent
            }
        };

        for event in reader.iter() {
            let interact_typ = match interact_type.get(event.with) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };
            if event.button == MouseButton::Left
                && viewmodel.holding() == ViewModelHold::LoCBundle
                && interact_typ == Interactable::LOC
                && interact_typ == Interactable::LOCG
            {
                let floor_item_children = match children.get(event.with) {
                    Ok(c) => c,
                    Err(_) => {
                        commands.entity(event.with).add_children(|_| {});
                        children.get(event.with).unwrap()
                    }
                };
                viewmodel.change_holding(ViewModelHold::Empty);
                state.change_state(PlayerState::Idle);
                // remove the viewmodel child
                commands.entity(vm_ent).remove_children(&[vm_child_id]);
                // get & remove the children of the viewmodel child
                let mut viewmodel_children_children = vm_children.to_vec();
                commands
                    .entity(vm_child_id)
                    .remove_children(&viewmodel_children_children);
                viewmodel_children_children.insert(0, vm_child_id);
                viewmodel_children_children.iter().for_each(|c| {
                    let mut ent = commands.entity(*c);
                    ent.insert(ActiveCollisionTypes::default());
                    ent.insert(Transform::from_xyz(
                        // FIXME: Adjust
                        0.0,
                        0.0,
                        -0.3 * (floor_item_children.len() + 1) as f32,
                    ));
                });
                commands
                    .entity(event.with)
                    .push_children(&viewmodel_children_children);
            }
        }
    }
    pub fn interact_mbleft_holdinghammer_interactwithloc(
        mut reader: EventReader<MouseInteraction>,
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
                println!("SWING (TODO!)");
                let ray_dir_y_inv = Vec3::new(ray_dir.x, -ray_dir.y, ray_dir.z);
                commands.entity(event.with).insert(ExternalImpulse {
                    impulse: ray_dir_y_inv * 10.0,
                    ..Default::default()
                });
            }
        }
    }

    pub fn interact_mbleft_holdinghammer_interactwithlocbundle(
        mut reader: EventReader<MouseInteraction>,
        mut commands: Commands,
        mut viewmodel_query: Query<&mut ViewModel, With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        transgender: Query<&Transform>,
        children: Query<&Children, (Without<Interactable>, Without<ViewModel>)>,
        parents: Query<&Parent, (Without<Interactable>, Without<ViewModel>)>,
    ) {
        let viewmodel = viewmodel_query.single_mut();

        for event in reader.iter() {
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
                    Err(_) => {
                        commands.entity(event.with).add_children(|_| {});
                        children.get(event.with).unwrap()
                    }
                };
                commands
                    .entity(event.with)
                    .remove_children(floor_item_children);
                let parent = parents.get(event.with).unwrap().get();

                let current_bundle_trans = transgender.get(event.with).unwrap();
                let mut random = SmallRng::from_entropy();
                for (i, e) in floor_item_children.iter().enumerate() {
                    let mut ent = commands.entity(*e);
                    ent.insert(Transform::from_xyz(
                        current_bundle_trans.translation.x,
                        (i as f32) * 0.3,
                        current_bundle_trans.translation.y,
                    ));
                    let random_x = random.gen_range(0.0..=1.0);
                    let random_y = random.gen_range(0.0..=1.0);
                    let random_z = random.gen_range(0.0..=1.0);
                    ent.insert(ExternalImpulse {
                        impulse: Vec3::new(random_x, random_y, random_z) * 10.0,
                        ..Default::default()
                    });
                    ent.insert(Interactable {
                        itype: InteractableType::LineOfCode,
                    });
                }
                commands.entity(parent).push_children(floor_item_children);
            }
        }
    }

    pub fn interact_mbleft_holdingany_interactterminal(
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
            }
        }
    }

    pub fn interact_mbleft_holdingnone_interactany(
        mut reader: EventReader<MouseInteraction>,
        mut commands: Commands,
        mut viewmodel_query: Query<(&mut ViewModel, Entity, &Children), With<ViewModel>>,
        interact_type: Query<&Interactable, Without<ViewModel>>,
        transgender: Query<&Transform>,
        parents: Query<&Parent, (Without<Interactable>, Without<ViewModel>)>,
    ) {
        let (mut viewmodel, vm_ent, vm_children) = viewmodel_query.single_mut();
        let vm_child_id: Entity = match vm_children.get(0) {
            Some(v) => *v,
            None => {
                let new_ent = commands.spawn().id();
                commands.entity(vm_ent).add_child(new_ent);
                new_ent
            }
        };
        for event in reader.iter() {
            let interact_typ = match interact_type.get(event.with) {
                Ok(inter) => *inter,
                Err(_) => continue,
            };
            if event.button == MouseButton::Left && viewmodel.holding() == ViewModelHold::Empty {
                let new_view_type = match interact_typ.itype() {
                    InteractableType::Hammer => ViewModelHold::Hammer,
                    InteractableType::LineOfCode => ViewModelHold::LoC,
                    InteractableType::LineOfCodeGlobule => ViewModelHold::LoCBundle,
                    InteractableType::Terminal => return,
                };

                viewmodel.change_holding(new_view_type);
                let current_ent_trans = transgender.get(event.with).unwrap();
                let parent_interacted_id = parents.get(event.with).unwrap().get();

                commands
                    .entity(parent_interacted_id)
                    .remove_children(&[event.with]);
                commands
                    .entity(event.with)
                    .insert(Transform::from_xyz(0.0, 0.0, 0.0))
                    .insert(ActiveCollisionTypes::empty());
                commands.entity(vm_ent).push_children(&[event.with]);
                commands.entity(vm_ent).remove_children(&[vm_child_id]);
                commands
                    .entity(vm_child_id)
                    .insert(*current_ent_trans)
                    .insert(ActiveCollisionTypes::default());
                commands
                    .entity(parent_interacted_id)
                    .push_children(&[vm_child_id]);
            }
        }
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
