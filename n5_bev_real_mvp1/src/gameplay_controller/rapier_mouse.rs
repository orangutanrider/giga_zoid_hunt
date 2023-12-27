use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::render::primitives::Aabb;

use crate::unit::*;
use crate::unit::selectable::*;

const UNIT_FILTER: QueryFilter = QueryFilter{
    flags: QueryFilterFlags::ONLY_KINEMATIC, 
    groups: None, 
    exclude_collider: None, 
    exclude_rigid_body: None, 
    predicate: None,
};

const SINGLE_CAST_RADIUS: f32 = 5.0;
pub fn single_cast (
    rapier: Res<RapierContext>,
    cast_position: Vec2,
) -> Option<(Entity, Toi)> {
    return rapier.cast_shape(
        cast_position, 
        0.0, 
        Vec2::ZERO, 
        &Collider::ball(SINGLE_CAST_RADIUS), 
        0.0, 
        UNIT_FILTER,
    );
}

pub fn multi_aabb_intersections (
    rapier: Res<RapierContext>,
    aabb: Aabb,
    callback: impl FnMut(Entity) -> bool, // for each intersecting aabb 
) {
    rapier.colliders_with_aabb_intersecting_aabb(
        aabb,
        callback,
    );
}

pub fn vec2s_to_aabb(vec1: Vec2, vec2: Vec2) -> Aabb {
    let mut max: Vec2 = Vec2::ZERO;
    // max X
    if vec1.x > vec2.x{
        max.x = vec1.x;
    } else{
        max.x = vec2.x;
    }
    // max y
    if vec1.y > vec2.y{
        max.y = vec1.y;
    }
    else{
        max.y = vec2.y;
    }

    let mut min = Vec2::ZERO;
    // min X
    if vec1.x < vec2.x{
        min.x = vec1.x;
    } else{
        min.x = vec2.x;
    }
    // min y
    if vec1.y < vec2.y{
        min.y = vec1.y;
    }
    else{
        min.y = vec2.y;
    }
    
    return Aabb::from_min_max(
        Vec3 { x: min.x, y: min.y, z: 0.0 },
        Vec3 { x: max.x, y: max.y, z: 0.0 },
    );
}