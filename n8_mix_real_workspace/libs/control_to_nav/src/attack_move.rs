use bevy::prelude::*;
use rts_unit_control::{prelude::*, validate_active_terminal_c};
use rts_unit_nav::*;

use crate::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

#[derive(Component)]
pub struct SwitchedNavAsAttackMove<S: RefSignature>{
    pub switch: bool,
    signature: PhantomData<S>
}
impl<S: RefSignature> SwitchedTransmissionFlag for SwitchedNavAsAttackMove<S> {
    fn set(&mut self, v: bool) {
        self.switch = v;
    }

    fn read(&self) -> bool {
        return self.switch
    }
}
impl<S: RefSignature> Default for SwitchedNavAsAttackMove<S> {
    fn default() -> Self {
        Self { switch: false, signature: Default::default() }
    }
}

pub fn reference_attack_move_as_reference_nav_sys<S: RefSignature>(
    q: Query<(&SwitchedNavAsAttackMove<S>, 
        &ToControl, // ideally would be a signed waymark
        &ToNav, // ideally would be a signed waymark
    ), (With<NavIsReference<S>>, With<ControlIsReference<S>>)>,
    attack_move_q: Query<(&TActiveOrderType, &TAttackMoveOrders)>,
    mut nav_q: Query<&mut TNavWaypoint>,
) {
    for (switch, to_control, to_nav) in q.iter() {
        if !switch.switch {
            continue;
        }
        reference_attack_move_as_reference_nav(to_control, to_nav, &attack_move_q, &mut nav_q);
    }
} 

fn reference_attack_move_as_reference_nav(
    to_control: &ToControl, 
    to_nav: &ToNav, 
    attack_move_q: &Query<(&TActiveOrderType, &TAttackMoveOrders)>,
    nav_q: &mut Query<&mut TNavWaypoint>,
) {
    ref_caravan!(
        to_control::attack_move_q((type_terminal, data_terminal));
        to_nav::nav_q(mut nav_terminal);
    );

    validate_active_terminal_r!(TAttackMoveOrders, type_terminal);
    
    let Some(current) = data_terminal.current() else {
        return;
    };
    nav_terminal.0 = current.waypoint;
}

/* 
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
*/