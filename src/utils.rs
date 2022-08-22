/// General utilities for automating away boilerplate
use crate::prelude::*;
use bevy::ecs::query::WorldQuery;

pub fn window_descriptor(width: f32, height: f32) -> WindowDescriptor {
    WindowDescriptor {
        title: format!(
            "British Fool Merging Corperation: With the GIT, All the DIFFs! - v{}",
            env!("CARGO_PKG_VERSION")
        ),
        width,
        height,
        ..default()
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Component)]
pub enum ColliderType {
    Static,
    Sensor,
    Dynamic,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Component)]
pub struct EName {
    pub id: String,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, WorldQuery)]
pub struct ColliderData {
    pub c_type: &'static ColliderType,
    pub id: &'static EName,
}
