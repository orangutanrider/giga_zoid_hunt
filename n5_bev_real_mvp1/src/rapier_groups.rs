use bevy_rapier2d::prelude::Group;
use bevy_rapier2d::prelude::QueryFilter;
use bevy_rapier2d::prelude::QueryFilterFlags;
use bevy_rapier2d::prelude::CollisionGroups;

// The groups membership indicates what groups the collider is part of (one bit per group).
// The groups filter indicates what groups the collider can interact with (one bit per group).

// https://rapier.rs/docs/user_guides/rust/colliders
// https://taintedcoders.com/bevy/rapier/#sensors 

// Groups
/// PLAYER: Attackable, Selectable
const P_NON_SOLID: Group = Group::GROUP_2; 
/// Prince unit
const PRINCE: Group = Group::GROUP_5;

/// ENEMY: Attackable
const E_NON_SOLID: Group = Group::GROUP_6;

// Collision group presets
const P_NON_SOLID_CGROUP: CollisionGroups = CollisionGroups::new(
    P_NON_SOLID,
    Group::NONE, 
);

const E_NON_SOLID_CGROUP: CollisionGroups = CollisionGroups::new(
    E_NON_SOLID,
    Group::NONE, 
);

// Filter presets
/// PLAYER: Attackable, Selectable
const P_NON_SOLID_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, 
    groups: Some(CollisionGroups::new(
        P_NON_SOLID,  // A filter can be both a member and a filter? I'm not sure if there is or is not a reason for this.
        P_NON_SOLID,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

/// ENEMY: Attackable
const E_NON_SOLID_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, 
    groups: Some(CollisionGroups::new(
        E_NON_SOLID, 
        E_NON_SOLID,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};