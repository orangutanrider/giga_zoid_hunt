use bevy::prelude::*;
use rts_unit_control::prelude::*;
use rts_unit_nav::*;

use crate::*;

#[derive(Component)]
/// Data transimission flag.
pub struct NavWhenIdleControl; 

/// Nav = NavWhenIdleControl + (NavIsLocal + ControlIsLocal)
pub fn local_control_attack_target_navigation_system(
    mut q: Query<(&mut TNavWaypoint, &ActiveOrderTerminal, &GlobalTransform), (Changed<ActiveOrderTerminal>, With<NavWhenIdleControl>, With<NavIsLocal>, With<ControlIsLocal>)>,
) {
    for (mut nav_input, order_type, transform) in q.iter_mut() {
        if order_type.current().is_none() {
            nav_input.0 = transform.translation().truncate();
        }
    }
} 