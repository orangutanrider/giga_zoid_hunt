use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

#[derive(Component)]
/// Data transimission flag.
pub struct NavAsActiveAttackMove; 

/// Nav = NavAsAttackMove + (NavIsLocal + ControlIsLocal)
pub fn local_control_attack_move_navigation_system(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &TAttackMoveOrders), (With<NavAsActiveAttackMove>, With<NavIsLocal>, With<ControlIsLocal>)>
) {
    for (mut nav_input, order_type, order_data) in q.iter_mut() {
        validate_active_terminal_c!(TAttackMoveOrders, order_type);
        let Some(order) = order_data.current() else {
            continue; 
        };
        nav_input.0 = order.waypoint;
    }
} 