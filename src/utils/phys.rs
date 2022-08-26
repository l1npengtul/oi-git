macro_rules! declare_groups {
    (
        $group:ident { $collide:ident, $interact:ident}
        $($name:ident($memberships:expr, $filter:expr)),* $(,)?
    ) => {
        pub mod $group {
            use super::*;
            declare_groups!(@interact $interact => $($name($memberships, $filter)),*);
            declare_groups!(@collide $collide => $($name($memberships, $filter)),*);
        }
    };
    (@interact $interact:ident => $($name:ident($memberships:expr, $filter:expr)),*) => {
        pub mod $interact {
            use super::*;
            $(
                #[allow(dead_code)]
                pub fn $name() -> ::bevy_rapier3d::geometry::InteractionGroups {
                    ::bevy_rapier3d::geometry::InteractionGroups::new($memberships, $filter)
                }
            )*
        }
    };
    (@collide $collide:ident =>  $($name:ident($memberships:expr, $filter:expr)),*) => {
        pub mod $collide {
            use super::*;
            $(
                #[allow(dead_code)]
                pub fn $name() -> ::bevy_rapier3d::geometry::CollisionGroups {
                    ::bevy_rapier3d::geometry::CollisionGroups::new($memberships, $filter)
                }
            )*
        }
    }
}

const ALL: u32 = u32::MAX;
const STATIC: u32 = 1;
const DYNAMIC: u32 = 1 << 1;
const PLAYER: u32 = 1 << 2;
const INTERACTIBLE: u32 = 1 << 3;
// Defines functions in 2 modules
// One builds CollisionGroups
// The other builds InteractionGroups
// syntax is groupname(MEMBERSHIP_1 | MEMBERSHIP_2, FILTER)
declare_groups!(
    group { collide, interact }
    all(ALL, ALL),
    static_body(STATIC | INTERACTIBLE, STATIC | PLAYER | DYNAMIC | INTERACTIBLE),
    player_body(PLAYER, DYNAMIC | STATIC),
    player_vision(INTERACTIBLE, INTERACTIBLE),
    dynamic_body(DYNAMIC, STATIC | DYNAMIC),
    interactable_body(INTERACTIBLE, INTERACTIBLE),
);

// the reason for this is that bitmasks like this are a footgun so i want
// to centralise those uses of bitmasks to make bitmask errors less stealthy
