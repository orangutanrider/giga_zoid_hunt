use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rapier_config::*;

use crate::*;

#[derive(Component)]
pub struct EnemyCircleIntersections{
    radius: f32,
}
impl EnemyCircleIntersections {
    pub fn new(radius: f32) -> Self {
        return Self {
            radius,
        }
    }
}
impl ImmutableDetector for EnemyCircleIntersections {
    const FILTER: QueryFilter<'static> = E_DETECTABLE_FILTER;
    
    fn shape(&self) -> Collider {
        return Collider::ball(self.radius)
    }
}

/// Detection to local aggregate terminal.
pub fn enemy_circle_intersections_sys(
    rapier: &RapierContext,
    mut q: Query<(&mut TIntersectionsAggregate, &GlobalTransform, &EnemyCircleIntersections)>
) {
    for (mut terminal, transform, params) in q.iter_mut() {
        terminal.0.clear();

        let shape_pos = transform.translation().truncate();
        let shape = &params.shape();
        let callback = |entity| -> bool {
            terminal.0.push(entity);
            return true
        };

        rapier.intersections_with_shape(shape_pos, 0.0, shape, EnemyCircleIntersections::FILTER, callback)
    }
}