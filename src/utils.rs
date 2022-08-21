/// General utilities for automating away boilerplate
use crate::prelude::*;

pub fn window_descriptor(width: f32, height: f32) -> WindowDescriptor {
    WindowDescriptor {
        title: format!(
            "{} - v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ),
        width,
        height,
        ..default()
    }
}
