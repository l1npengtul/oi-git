use crate::{config::PlayerConfig, prelude::*};

pub fn build(app: &mut App) {
    app.add_system(Player::movement.run_in_state(GameState::MainMenu));
}

impl Player {
    pub fn movement(
        keys: Res<Input<KeyCode>>,
        time: Res<Time>,
        windows: Res<Windows>,
        settings: Res<PlayerConfig>,
        mut query: Query<&mut Transform, With<Player>>,
    ) {
        let window = windows.get_primary().unwrap();
        if !window.cursor_locked() {
            return;
        }

        let mut player_trans = query.single_mut();

        let local_z = player_trans.local_z();
        let fwd = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);
        let up = Vec3::Y;

        let mut vel = Vec3::ZERO;
        for key in keys.get_pressed() {
            vel = vel
                + match key {
                    KeyCode::W => fwd,
                    KeyCode::S => -fwd,
                    KeyCode::A => -right,
                    KeyCode::D => right,
                    KeyCode::Space => up,
                    KeyCode::LShift | KeyCode::LControl => -up,
                    _ => continue,
                }
        }

        vel = vel.normalize_or_zero();

        player_trans.translation += vel * time.delta_seconds() * settings.mvmnt_speed;
    }
}
