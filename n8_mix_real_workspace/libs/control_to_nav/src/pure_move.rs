use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

#[derive(Component)]
/// Data transimission flag.
pub struct NavAsActivePureMove; 

/// Nav = NavAsPureMove + (NavIsLocal + ControlIsLocal)
pub fn local_control_pure_move_navigation_system(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &TPureMoveOrders), (With<NavAsActivePureMove>, With<NavIsLocal>, With<ControlIsLocal>)>
) {
    for (mut nav_input, order_type, order_data) in q.iter_mut() {
        validate_active_terminal_c!(TPureMoveOrders, order_type);
        let Some(order) = order_data.current() else {
            continue; 
        };
        nav_input.0 = order.waypoint;
    }
} 