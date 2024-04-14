use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;


#[derive(Component)]
pub struct SwitchedNavAsAttackTarget<S: RefSignature>{
    pub switch: bool,
    signature: PhantomData<S>
}
impl<S: RefSignature> SwitchedTransmissionFlag for SwitchedNavAsAttackTarget<S> {
    fn set(&mut self, v: bool) {
        self.switch = v;
    }

    fn read(&self) -> bool {
        self.switch
    }
}
impl<S: RefSignature> Default for SwitchedNavAsAttackTarget<S> {
    fn default() -> Self {
        Self { switch: false, signature: Default::default() }
    }
}

pub fn reference_attack_target_as_reference_nav_sys<S: RefSignature>(
    q: Query<(&ToNav, &ToControl, &SwitchedNavAsAttackTarget<S>), (With<NavIsReference<S>>, With<ControlIsReference<S>>)>,
    attack_target_q: Query<(&TActiveOrderType, &TCurrentTarget)>,
    mut nav_q: Query<&mut TNavWaypoint>,
    target_q: Query<&GlobalTransform>,
) {
    for (to_nav, to_control, switch) in q.iter() {
        if !switch.switch {
            continue;
        }
        reference_attack_target_as_reference_nav(to_control, to_nav, &attack_target_q, &target_q, &mut nav_q);
    }
} 

fn reference_attack_target_as_reference_nav(
    to_control: &ToControl, 
    to_nav: &ToNav, 
    attack_target_q: &Query<(&TActiveOrderType, &TCurrentTarget)>,
    target_q: &Query<&GlobalTransform>,
    nav_q: &mut Query<&mut TNavWaypoint>,
) {
    ref_caravan!(
        to_control::attack_target_q((type_terminal, data_terminal));
        to_nav::nav_q(mut nav_terminal);
    );

    validate_active_terminal_r!(TAttackTargetOrders, type_terminal);
    
    let Some(target) = data_terminal.read() else {
        return; 
    };
    let Ok(transform) = target_q.get(target) else {
        return;
    };
    let waypoint = transform.translation().truncate();
    nav_terminal.0 = waypoint;
}

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