use bevy::prelude::*;
use rts_unit_control::prelude::*;
use rts_unit_nav::*;

use crate::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

/* 
#[derive(Component)]
/// Data transimission flag.
pub struct NavWhenIdleControl<S: RefSignature>{
    signature: PhantomData<S>
} 

/// Nav = NavWhenIdleControl + (NavIsLocal + ControlIsLocal)
pub fn local_control_attack_target_navigation_system<S: RefSignature>(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &GlobalTransform), (Changed<ActiveOrderTerminal>, With<NavWhenIdleControl<S>>, With<NavIsLocal<S>>, With<ControlIsLocal<S>>)>,
) {
    for (mut nav_input, order_type, transform) in q.iter_mut() {
        if order_type.current().is_none() {
            nav_input.0 = transform.translation().truncate();
        }
    }
} 
*/