use crate::config::PlayerConfig;
use crate::player::fsm::{PlayerState, PlayerStateMachine};
use crate::player::{Player, PlayerCamera};
use crate::prelude::*;
use crate::viewmodel::ViewModel;
use bevy::{
    ecs::event::{Events, ManualEventReader},
    input::mouse::MouseMotion,
};

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Default)]
pub struct MouseInputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub fn build(app: &mut App) {
    app.init_resource::<MouseInputState>()
        .add_system(
            Player::look
                .run_in_state(GameState::MainMenu)
                .run_unless_resource_equals(PlayerStateMachine::INTERACTING),
        )
        .add_system(Player::sync_locations.run_in_state(GameState::MainMenu));
}

impl Player {
    pub fn look(
        config: Res<PlayerConfig>,
        windows: Res<Windows>,
        mut state: ResMut<MouseInputState>,
        motion: Res<Events<MouseMotion>>,
        mut query: Query<&mut Transform, With<PlayerCamera>>,
    ) {
        let window = windows.get_primary().unwrap();

        let mut delta_state = state.as_mut();
        let mut player_trans = query.single_mut(); // owo

        for ev in delta_state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                delta_state.pitch -= (config.mouse_sens * ev.delta.y * window_scale).to_radians();
                delta_state.yaw -= (config.mouse_sens * ev.delta.x * window_scale).to_radians();
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            player_trans.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    }

    pub fn sync_locations(
        mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
        mut viewmodel_query: Query<
            &mut Transform,
            (With<ViewModel>, Without<Player>, Without<PlayerCamera>),
        >,
        mut player_query: Query<&Transform, With<Player>>,
    ) {
        let mut camera = camera_query.single_mut();
        let mut viewmodel = viewmodel_query.single_mut();
        let player = player_query.single_mut();

        let mut cm_trans = player.translation;
        cm_trans.y += 0.4;
        camera.translation = cm_trans;

        let vm_trans = Vec3::new(0.0, -0.5, -1.0);
        let c_rot = camera.rotation;
        let fin = (c_rot * vm_trans).normalize_or_zero() * 2.0;

        viewmodel.translation = fin + camera.translation;
        viewmodel.rotation = camera.rotation;
    }
}
