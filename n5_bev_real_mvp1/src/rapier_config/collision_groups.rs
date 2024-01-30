use bevy_rapier2d::prelude::{
    Group,
    CollisionGroups,
};

use super::groups::*;

// membership indicates what groups the collider is part of.
// filter indicates what groups the collider can interact with.

// Player team collsion groups
pub const P_SELECTABLE_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SELECTABLE,
    Group::NONE, 
);

pub const P_ATTACKABLE_CGROUP: CollisionGroups = CollisionGroups::new(
    P_ATTACKABLE,
    Group::NONE, 
);

// Enemy team collision groups
pub const E_ATTACKABLE_CGROUP: CollisionGroups = CollisionGroups::new(
    E_ATTACKABLE,
    Group::NONE, 
);