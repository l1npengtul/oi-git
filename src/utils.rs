/// General utilities for automating away boilerplate
use crate::prelude::*;
use bevy::ecs::query::WorldQuery;

pub fn window_descriptor(width: f32, height: f32) -> WindowDescriptor {
    WindowDescriptor {
        title: format!(
            "Oi! Git!: GIT Co TRADE SECRET - v{}",
            env!("CARGO_PKG_VERSION")
        ),
        width,
        height,
        ..default()
    }
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ($e:expr $(; else $fail:expr)?) => {
        match $e {
            ::std::option::Option::Some(v) => v,
            ::std::option::Option::None => {
                $($fail;)?
                continue
            }
        }
    };
}
pub use crate::unwrap_or_continue;
/// Doesn't work without prelude in scope.
/// just put build_world_access_macros!(world, res, assets);
/// and you're in business
#[macro_export]
macro_rules! build_world_access_macros {
    ($world:ident, $res_name:ident, $asset_name:ident $(,)?) => {
        macro_rules! $res_name {
            ($res:ty) => {
                $world.resource::<$res>()
            };
        }
        macro_rules! $asset_name {
            ($assets:ty) => {
                $res_name!(Assets<$assets>)
            };
        }
    };
}
pub use crate::build_world_access_macros;

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

pub mod phys;
