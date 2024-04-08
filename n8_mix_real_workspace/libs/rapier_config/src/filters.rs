use bevy_rapier2d::prelude::{
    QueryFilter,
    CollisionGroups,
    QueryFilterFlags,
};

use super::groups::*;

pub const SELECTABLE_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        SELECTABLE, 
        SELECTABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

pub const HITTABLE_ENEMY_UNITS_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        E_HITTABLE, 
        E_HITTABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

pub const HITTABLE_PLAYER_UNITS_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        P_HITTABLE, 
        P_HITTABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

pub const DETECTABLE_ENEMY_UNITS_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        E_DETECTABLE, 
        E_DETECTABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

pub const DETECTABLE_PLAYER_UNITS_FILTER: QueryFilter = QueryFilter { 
    flags: QueryFilterFlags::EXCLUDE_SOLIDS, // Sensors only
    groups: Some(CollisionGroups::new(
        P_DETECTABLE, 
        P_DETECTABLE,
    )), 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};