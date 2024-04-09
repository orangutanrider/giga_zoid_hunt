use bevy::prelude::*;

use super::*;

#[derive(Component, Default)]
/// Data process flag.
/// Makes the navigation vector, point directly to the waypoint input.
pub struct DirectNav;

pub fn direct_nav_sys(
    mut q: Query<(&mut NavVectorOutput, &TNavWaypoint, &GlobalTransform), With<DirectNav>>,
) {
    for (mut output, input, transform) in q.iter_mut() {
        let waypoint = input.0;
        let vector = waypoint - transform.translation().truncate();
        let normalized_vector = vector.normalize_or_zero();
        output.0 = normalized_vector;
    }
}