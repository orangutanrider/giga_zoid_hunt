use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

#[derive(Component)]
/// Data transimission flag.
pub struct NavAsActiveAttackMove<S: RefSignature>{
    signature: PhantomData<S>
} 

/// Nav = NavAsAttackMove + (NavIsLocal + ControlIsLocal)
pub fn local_control_attack_move_navigation_system<S: RefSignature>(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &TAttackMoveOrders), (With<NavAsActiveAttackMove<S>>, With<NavIsLocal<S>>, With<ControlIsLocal<S>>)>
) {
    for (mut nav_input, order_type, order_data) in q.iter_mut() {
        validate_active_terminal_c!(TAttackMoveOrders, order_type);
        let Some(order) = order_data.current() else {
            continue; 
        };
        nav_input.0 = order.waypoint;
    }
} 