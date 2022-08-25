use crate::player::fsm::{PlayerState, PlayerStateMachine};
use crate::{config::PlayerConfig, prelude::*};
use bevy_rapier3d::prelude::Velocity;

use super::PlayerCamera;

pub fn build(app: &mut App) {
    app.add_system(
        Player::movement
            .run_in_state(GameState::MainMenu)
            .run_unless_resource_equals(PlayerStateMachine::INTERACTING),
    );
    app.add_system(
        Player::escape
            .run_in_state(GameState::MainMenu)
            .run_if_resource_equals(PlayerStateMachine::INTERACTING),
    );
}

impl Player {
    pub fn movement(
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
        windows: Res<Windows>,
        settings: Res<PlayerConfig>,
        camera_query: Query<&Transform, With<PlayerCamera>>,
        mut player_query: Query<&mut Velocity, With<Player>>,
    ) {
        let window = windows.get_primary().unwrap();
        if !window.cursor_locked() {
            return;
        }

        let mut player_vel = player_query.single_mut();

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

        player_vel.linvel = vel;
    }

    pub fn escape(mb: Res<Input<MouseButton>>, mut state: ResMut<PlayerStateMachine>) {
        for key in keys.get_pressed() {
            match key {
                KeyCode::Escape | KeyCode::Space => {
                    state.change_state(PlayerState::Idle);
                    break;
                }
                _ => continue,
            }
        }
    }
}
