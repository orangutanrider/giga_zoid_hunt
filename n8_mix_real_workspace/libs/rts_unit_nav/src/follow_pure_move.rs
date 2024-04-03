use bevy::prelude::*;
use rts_unit_control::prelude::*;

use crate::*;

#[derive(Component)]
pub struct PureMoveNavigation;

pub fn pure_move_navigation_system(
    mut q: Query<(&mut NavVectorOutput, &GlobalTransform, &TNavPureMove, &TNavType), With<PureMoveNavigation>>
) {
    for (mut output, transform, order_data, order_type) in q.iter_mut() {
        c_validate_data_terminal!(PureMoveOrder, order_type);

        let vector = order_data.0.waypoint;
        let vector = vector - transform.translation().truncate();
        let vector = vector.normalize();

        output.0 = vector;
    }
}