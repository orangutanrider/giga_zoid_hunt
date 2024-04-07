use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

#[derive(Component)]
/// Data transimission flag.
pub struct NavAsActiveAttackTarget; 

/// Nav = NavAsActiveAttackTarget + (NavIsLocal + ControlIsLocal)
pub fn local_control_attack_target_navigation_system(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &CurrentTarget), (With<NavAsActiveAttackTarget>, With<NavIsLocal>, With<ControlIsLocal>)>,
    target_q: Query<&GlobalTransform>,
) {
    for (mut nav_input, order_type, order_data) in q.iter_mut() {
        validate_active_terminal_c!(TAttackTargetOrders, order_type);
        let Some(target) = order_data.read() else {
            continue; 
        };
        let Ok(transform) = target_q.get(target) else {
            continue;
        };
        let waypoint = transform.translation().truncate();
        nav_input.0 = waypoint;
    }
} 