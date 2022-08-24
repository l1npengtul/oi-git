use crate::{
    config::PlayerConfig,
    prelude::{phys::*, *},
};

use super::PlayerCamera;

pub struct MouseInteraction {
    pub button: MouseButton,
    pub with: Entity,
    pub toi: f32,
}

#[derive(Default)]
pub struct PlayerLookingAt {
    pub entity: Option<Entity>,
    pub dist: f32,
}

pub fn build(app: &mut App) {
    app.init_resource::<PlayerLookingAt>()
        .add_event::<MouseInteraction>()
        .add_system(MouseInteraction::detect.run_in_state(GameState::MainMenu));
}

impl MouseInteraction {
    pub fn detect(
        player_config: Res<PlayerConfig>,
        mut interacts: EventWriter<MouseInteraction>,
        bttns: Res<Input<MouseButton>>,
        rapier: Res<RapierContext>,
        player_query: Query<&Transform, With<PlayerCamera>>,
        mut looking_at: ResMut<PlayerLookingAt>,
    ) {
        let pressed = bttns.get_just_pressed();
        let player_trans = player_query.single();
        let ray_origin = player_trans.translation;
        let ray_dir = (player_trans.rotation * -Vec3::Z).normalize_or_zero();
        let max_toi = player_config.reach_dist;
        let solid = false;
        let groups = group::interact::player_vision();
        let filter = groups.into();
        if let Some((entity, toi)) = rapier.cast_ray(ray_origin, ray_dir, max_toi, solid, filter) {
            *looking_at = PlayerLookingAt {
                entity: Some(entity),
                dist: toi,
            };
            pressed.for_each(|button| {
                interacts.send(MouseInteraction {
                    with: entity,
                    button: button.clone(),
                    toi,
                })
            });
        } else {
            looking_at.entity = None
        }
    }
}
