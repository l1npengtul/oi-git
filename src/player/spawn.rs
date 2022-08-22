use crate::prelude::*;

pub fn build(app: &mut App) {
    app.add_enter_system(GameState::MainMenu, Player::spawn);
}

#[derive(Bundle)]
pub struct PlayerBundle {
    this: Player,
    #[bundle]
    camera: Camera3dBundle,
}

impl Player {
    pub fn spawn(mut commands: Commands) {
        commands.spawn_bundle(PlayerBundle {
            this: Player,
            camera: Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(-1.0, 0.0, 0.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            },
        });
    }
}
