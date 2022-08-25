use crate::player::fsm::{PlayerState, PlayerStateMachine};
use crate::{config::PlayerConfig, prelude::*};
use bevy_rapier3d::prelude::Velocity;

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
        camera_query: Query<&Transform, With<PlayerCamera>>,
        mut player_query: Query<(&mut Velocity, &mut PlayerStateMachine), With<Player>>,
    ) {
        let window = windows.get_primary().unwrap();
        if !window.cursor_locked() {
            return;
        }

        let (mut player_vel, mut player_sm) = player_query.single_mut();

        let mut move_speed = settings.mvmnt_speed;
        match player_sm.state() {
            PlayerState::Idle => player_sm.change_state(PlayerState::Walking),
            PlayerState::Interacting => move_speed = 0.0,
            _ => {}
        }

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
                KeyCode::Escape => {
                    if player_sm.state() == PlayerState::Interacting {
                        player_sm.change_state(PlayerState::Idle);
                    }
                    return;
                }
                _ => continue,
            }
        }

        vel = vel.normalize_or_zero() * time.delta_seconds() * settings.mvmnt_speed;

        player_vel.linvel = vel;
    }
}
