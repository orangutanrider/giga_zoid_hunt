use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

/* 
#[derive(Component)]
pub struct SwitchedNavAsAttackTarget<S: RefSignature>{
    pub switch: bool,
    signature: PhantomData<S>
}

pub fn reference_attack_target_as_reference_nav_sys<S: RefSignature>(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &CurrentTarget, &SwitchedNavAsAttackTarget<S>), (With<NavIsReference<S>>, With<ControlIsReference<S>>)>,
    target_q: Query<&GlobalTransform>,
) {
    for (mut nav_input, order_type, order_data, switch) in q.iter_mut() {
        if !switch.switch {
            continue;
        }

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
*/

/* 
#[derive(Component)]
/// Data transimission flag.
pub struct NavAsActiveAttackTarget<S: RefSignature>{
    signature: PhantomData<S>
} 

/// Nav = NavAsActiveAttackTarget + (NavIsLocal + ControlIsLocal)
pub fn local_control_attack_target_navigation_system<S: RefSignature>(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &CurrentTarget), (With<NavAsActiveAttackTarget<S>>, With<NavIsLocal<S>>, With<ControlIsLocal<S>>)>,
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
*/