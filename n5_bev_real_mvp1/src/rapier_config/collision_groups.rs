use bevy_rapier2d::prelude::{
    Group,
    CollisionGroups,
};

use super::groups::*;

// membership indicates what groups the collider is part of.
// filter indicates what groups the collider can interact with.

pub const RTS_PHYSICS_CGROUP: CollisionGroups = CollisionGroups::new(
    RTS_PHYSICS, 
    RTS_PHYSICS
);

// Player team collsion groups
pub const P_CONTROL_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SELECTABLE,
P_SELECTABLE, 
);

pub const P_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SOUL,
    P_SOUL, 
);

// Prince
pub const PRINCE_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SOUL.union(P_PRINCE),
    P_SOUL.union(P_PRINCE), 
);

// Enemy team collision groups
pub const E_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    E_ATTACKABLE,
    E_ATTACKABLE, 
);