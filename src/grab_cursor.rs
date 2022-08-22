use crate::prelude::*;

pub struct GrabCursorPlugin;

impl Plugin for GrabCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cursor_grab);
    }
}

fn set_cursor_grabbed(window: &mut Window, grabbed: bool) {
    window.set_cursor_lock_mode(grabbed);
    window.set_cursor_visibility(!grabbed);
}

fn cursor_grab(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        set_cursor_grabbed(window, true);
    }

    if key.just_pressed(KeyCode::Escape) {
        set_cursor_grabbed(window, false);
    }
}
