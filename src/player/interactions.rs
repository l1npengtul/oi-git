use crate::player::fsm::{PlayerState, PlayerStateMachine};
use crate::player::PlayerCamera;
use crate::{
    config::PlayerConfig,
    prelude::{phys::*, *},
};

pub struct MouseInteraction {
    button: MouseButton,
    with: Entity,
    toi: f32,
}

pub fn build(app: &mut App) {
    app.add_event::<MouseInteraction>()
        .add_system(MouseInteraction::detect.run_in_state(GameState::MainMenu));
}

impl MouseInteraction {
    pub fn detect(
        player_config: Res<PlayerConfig>,
        mut interacts: EventWriter<MouseInteraction>,
        bttns: Res<Input<MouseButton>>,
        rapier: Res<RapierContext>,
        player_query: Query<&PlayerStateMachine, With<Player>>,
        camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    ) {
        let player_sm = player_query.single();
        let camera_trans = camera_query.single();

        if player_sm.state() == PlayerState::Interacting {
            return;
        }

        let mut pressed = bttns.get_just_pressed().peekable();
        if pressed.peek().is_none() {
            return;
        }
        // lmb has been pressed
        let ray_origin = camera_trans.translation;
        let ray_dir = camera_trans.rotation * -Vec3::Z;
        let max_toi = player_config.reach_dist;
        let solid = false;
        let groups = group::interact::player_vision();
        let filter = groups.into();
        if let Some((entity, toi)) = rapier.cast_ray(ray_origin, ray_dir, max_toi, solid, filter) {
            println!("interacted");
            pressed.for_each(|button| {
                interacts.send(MouseInteraction {
                    with: entity,
                    button: *button,
                    toi,
                })
            });
        }
    }

    pub fn on_interaction()
}
