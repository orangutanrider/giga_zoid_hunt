use bevy_rapier2d::prelude::{
    QueryFilter,
    CollisionGroups,
    QueryFilterFlags,
};

use super::groups::*;

// membership indicates what groups the collider is part of.
// filter indicates what groups the collider can interact with.

// Player team filters
pub const P_SELECTABLE_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        P_SELECTABLE, 
        P_SELECTABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

pub const P_ATTACKABLE_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        P_ATTACKABLE, 
        P_ATTACKABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

// Enemy team filters
pub const E_ATTACKABLE_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        E_ATTACKABLE, 
        E_ATTACKABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};