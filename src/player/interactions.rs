use crate::interactable::{Interactable, InteractableType};
use crate::player::fsm::{PlayerState, PlayerStateMachine};
use crate::player::PlayerCamera;
use crate::viewmodel::{ViewModel, ViewModelHold};
use crate::{
    config::PlayerConfig,
    prelude::{phys::*, *},
};
use nanorand::{Rng, WyRand};

pub struct MouseInteraction {
    button: MouseButton,
    with: Entity,
    toi: f32,
}

pub fn build(app: &mut App) {
    app.add_event::<MouseInteraction>()
        .add_system(MouseInteraction::interact.run_in_state(GameState::MainMenu));
}

impl MouseInteraction {
    pub fn interact(
        player_config: Res<PlayerConfig>,
        // mut interacts: EventWriter<MouseInteraction>,
        bttns: Res<Input<MouseButton>>,
        rapier: Res<RapierContext>,
        mut world: ResMut<World>,
        mut player_query: Query<&mut PlayerStateMachine, With<Player>>,
        camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
        mut viewmodel_query: Query<(&mut ViewModel, Entity), With<ViewModel>>,
    ) {
        let mut player_sm = player_query.single_mut();
        let camera_trans = camera_query.single();
        let (mut viewmodel, vm_ent) = viewmodel_query.single_mut();

        if player_sm.state() == PlayerState::Interacting {
            return;
        }

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

        let viewmodel_children = world.get_mut::<Children>(vm_ent).unwrap();
        let mut viewmodel_child = world.get_entity_mut(viewmodel_children[0]).unwrap();
        let mut viewmodel_mut = world.get_entity_mut(vm_ent).unwrap();

        if let Some((entity, toi)) = rapier.cast_ray(ray_origin, ray_dir, max_toi, solid, filter) {
            let mut parent_interated_ent = world.get_entity_mut(entity).unwrap();
            let floor_item_children = world.get::<Children>(entity).unwrap();
            let mut entity_mut = world.get_entity_mut(entity).unwrap();
            let interact_typ = world.get::<Interactable>(entity).unwrap().itype();

            match pressed.peek().unwrap() {
                // entity parent

                // safe since we know one has been pressed at least
                // Interact
                MouseButton::Left => {
                    // see what we are holding and what we just interacted with
                    // if we are holding a block/group and interact with a block or a group, we create/add to said block
                    // -> do this by parenting the block
                    // -> the ground item always takes priority
                    // if we are holding a hammer ad interact with a block, we send it flying
                    // -> Apply impulse to dynamic body
                    // if we are holding a hammer and interact with a group, we explode its components
                    // -> remove all its children, add to root with random offsets of height + random impulse
                    // if we are holding nothing, we pick up the item.
                    // if we interact with a terminal, enter the "Interacting" player state and tween the player's camera to
                    // the terminal's `point3d`.

                    // start by checking what the fuck we just interacted with
                    // FIXME: The collider must be readjusted to accomodate for children may be interacted with!
                    match (viewmodel.holding(), interact_typ) {
                        (
                            ViewModelHold::LoC,
                            InteractableType::LineOfCode | InteractableType::LineOfCodeGlobule,
                        ) => {
                            viewmodel.change_holding(ViewModelHold::Empty);
                            // remove the viewmodel child
                            viewmodel_mut.remove_children(&[viewmodel_child.id()]);
                            viewmodel_child.insert(ActiveCollisionTypes::default());
                            viewmodel_child.insert(Transform::from_xyz(
                                // FIXME: Adjust
                                0.0,
                                0.0,
                                -0.3 * (floor_item_children.len() + 1) as f32,
                            ));
                            entity_mut.push_children(&[viewmodel_child.id()]);
                        }
                        (
                            ViewModelHold::LoCBundle,
                            InteractableType::LineOfCode | InteractableType::LineOfCodeGlobule,
                        ) => {
                            viewmodel.change_holding(ViewModelHold::Empty);
                            // remove the viewmodel child
                            viewmodel_mut.remove_children(&[viewmodel_child.id()]);
                            // get & remove the children of the viewmodel child
                            let mut viewmodel_children_children = world
                                .get::<Children>(viewmodel_child.id())
                                .unwrap()
                                .as_ref()
                                .to_vec();
                            viewmodel_child.remove_children(&viewmodel_children_children);
                            viewmodel_children_children.insert(0, viewmodel_child.id());
                            viewmodel_children_children.iter().for_each(|c| {
                                let mut ent = world.get_entity_mut(*c).unwrap();
                                ent.insert(ActiveCollisionTypes::default());
                                ent.insert(Transform::from_xyz(
                                    // FIXME: Adjust
                                    0.0,
                                    0.0,
                                    -0.3 * (floor_item_children.len() + 1) as f32,
                                ));
                            });
                            entity_mut.push_children(&viewmodel_children_children);
                        }
                        (ViewModelHold::Hammer, InteractableType::LineOfCode) => {
                            println!("SWING (TODO!)");
                            let ray_dir_y_inv = Vec3::new(ray_dir.x, -ray_dir.y, ray_dir.z);
                            entity_mut.insert(ExternalImpulse {
                                impulse: ray_dir_y_inv * 10.0,
                                ..Default::default()
                            });
                        }
                        (ViewModelHold::Hammer, InteractableType::LineOfCodeGlobule) => {
                            // remove all children
                            entity_mut.remove_children(floor_item_children);
                            let current_bundle_trans = entity_mut.get::<Transform>().unwrap();
                            let mut random = WyRand::new();
                            for (i, e) in floor_item_children.iter().enumerate() {
                                let mut ent = world.get_entity_mut(*e).unwrap();
                                ent.insert(Transform::from_xyz(
                                    current_bundle_trans.translation.x,
                                    (i as f32) * 0.3,
                                    current_bundle_trans.translation.y,
                                ));
                                let random_x = (random.generate::<u8>() as f32 / 255.0);
                                let random_y = (random.generate::<u8>() as f32 / 255.0);
                                let random_z = (random.generate::<u8>() as f32 / 255.0);
                                ent.insert(ExternalImpulse {
                                    impulse: Vec3::new(random_x, random_y, random_z) * 10.0,
                                    ..Default::default()
                                });
                                ent.insert(Interactable {
                                    itype: InteractableType::LineOfCode,
                                });
                                parent_interated_ent.push_children(&[*e]);
                            }
                        }
                        (_, InteractableType::Terminal) => {
                            player_sm.change_state(PlayerState::Interacting);
                            println!("TODO");
                        }
                        (_, it) => {
                            let new_view_type = match it {
                                InteractableType::Hammer => ViewModelHold::Hammer,
                                InteractableType::LineOfCode => ViewModelHold::LoC,
                                InteractableType::LineOfCodeGlobule => ViewModelHold::LoCBundle,
                                InteractableType::Terminal => return,
                            };

                            viewmodel.change_holding(new_view_type);

                            // unparent from root and add to viewmodel
                            parent_interated_ent.remove_children(&[entity]);
                            entity_mut.insert(Transform::from_xyz(0.0, 0.0, 0.0));
                            entity_mut.insert(ActiveCollisionTypes::empty());
                            viewmodel_mut.push_children(&[entity]);
                        }
                    }
                }
                // Swap
                MouseButton::Right => {
                    // Swap held item with floor item.
                    // return if terminal
                    // do normal pickup code if we have nothing held
                    match (viewmodel.holding(), interact_typ) {
                        (_, InteractableType::Terminal) => return,
                        (ViewModelHold::Empty, it) => {
                            let new_view_type = match it {
                                InteractableType::Hammer => ViewModelHold::Hammer,
                                InteractableType::LineOfCode => ViewModelHold::LoC,
                                InteractableType::LineOfCodeGlobule => ViewModelHold::LoCBundle,
                                InteractableType::Terminal => return,
                            };

                            viewmodel.change_holding(new_view_type);

                            // unparent from root and add to viewmodel
                            parent_interated_ent.remove_children(&[entity]);
                            entity_mut.insert(Transform::from_xyz(0.0, 0.0, 0.0));
                            entity_mut.insert(ActiveCollisionTypes::empty());
                            viewmodel_mut.push_children(&[entity]);
                        }
                        (_, it) => {
                            let new_view_type = match it {
                                InteractableType::Hammer => ViewModelHold::Hammer,
                                InteractableType::LineOfCode => ViewModelHold::LoC,
                                InteractableType::LineOfCodeGlobule => ViewModelHold::LoCBundle,
                                InteractableType::Terminal => return,
                            };

                            viewmodel.change_holding(new_view_type);

                            let current_ent_trans = entity_mut.get::<Transform>().unwrap();
                            // unparent from root and add to viewmodel
                            parent_interated_ent.remove_children(&[entity]);
                            entity_mut.insert(Transform::from_xyz(0.0, 0.0, 0.0));
                            entity_mut.insert(ActiveCollisionTypes::empty());
                            viewmodel_mut.push_children(&[entity]);
                            viewmodel_mut.remove_children(&[viewmodel_child.id()]);
                            viewmodel_child.insert(*current_ent_trans);
                            viewmodel_child.insert(ActiveCollisionTypes::default());
                            parent_interated_ent.push_children(&[viewmodel_child.id()]);
                        }
                    }
                }
                _ => return,
            }
        } else {
            // throw if right click
            // if left click: swing hammer, or drop
            match pressed.peek().unwrap() {
                // safe since we know one has been pressed at least
                // S W I N G
                MouseButton::Left => {
                    match viewmodel.holding() {
                        ViewModelHold::Hammer => {
                            // swing
                        }
                        ViewModelHold::Empty => return,
                        _ => { //drop
                        }
                    }
                }
                // Y E E T held item
                MouseButton::Right => {
                    println!("YEET (TODO!)");
                    viewmodel.change_holding(ViewModelHold::Empty);
                    viewmodel_mut.remove_children(&[viewmodel_child.id()]);
                }
                _ => return,
            }
        }
    }
}
