use crate::phys::group::interact::player_body;
use crate::phys::TOIStatus;
use crate::{config::PlayerConfig, prelude::*};
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::{Collider, QueryFilter, Velocity};
use std::ops::Mul;

use super::PlayerCamera;

pub fn build(app: &mut App) {
    app.add_system(Player::movement.run_in_state(GameState::MainMenu));
}

impl Player {
    pub fn movement(
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
        windows: Res<Windows>,
        settings: Res<PlayerConfig>,
        rapier: Res<RapierContext>,
        camera_query: Query<&Transform, With<PlayerCamera>>,
        mut player_query: Query<(&mut Velocity, &Collider, &Transform), With<Player>>,
    ) {
        let window = windows.get_primary().unwrap();
        if !window.cursor_locked() {
            return;
        }

        let (mut player_vel, collider, player_pos) = player_query.single_mut();
        let camera_trans = camera_query.single();

        let local_z = camera_trans.local_z();
        let fwd = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        let mut vel = Vec3::ZERO;
        for key in keys.get_pressed() {
            vel += match key {
                KeyCode::W => fwd,
                KeyCode::S => -fwd,
                KeyCode::A => -right,
                KeyCode::D => right,
                _ => continue,
            }
        }

        vel = vel.normalize_or_zero() * time.delta_seconds() * settings.mvmnt_speed;

        let mut calc_trans = player_pos.translation;
        calc_trans.y += 5.0 * f32::EPSILON;
        // now we do a sweep
        match rapier.cast_shape(
            calc_trans,
            player_pos.rotation,
            vel,
            collider,
            time.delta_seconds(),
            QueryFilter::new().groups(player_body()),
        ) {
            Some((entity, toi)) => {
                dbg!("collision with {:?} : {:?}", entity.id(), toi);
                let new_vel = toi.normal1.dot(vel);
                player_vel.linvel = new_vel;
            }
            None => {
                player_vel.linvel = vel;
            }
        }
    }
}
