use std::any::TypeId;
use bevy::prelude::*;

use ref_paths::*;
use ref_caravan::*;

use rts_unit_nav::*;
use rts_unit_control::prelude::*;
use rts_waymarks::*;

// Data transfer flag(s).
// Combine with reference flag.
// (What data?)

#[derive(Component)]
/// Data Transfer Flag.
/// Combine with reference flag.
pub struct NavAsCurrentOrderInControl;
// You could have one of these per terminal.

// Reference flag(s)
// Combine with data transfer flag.
// (Where to get the data from.)

#[derive(Component)]
/// Reference Flag.
/// Combine with data transfer flag.
/// (Nav -> Root -> Control)
pub struct NavOrderFromControlViaRoot;

// Example reference flags
/* 

#[derive(Component)]
pub struct NavOrderFromLocal;

#[derive(Component)]
pub struct NavOrderFromDirect(Entity);
impl NavOrderFromDirect {...}

*/

// Data transfer from reference, Systems matrix

/// TNavType = NavAsCurrentOrderInControl + NavOrderFromControlViaRoot
pub fn nav_type_as_current_from_control_via_root_sys(
    mut nav_q: Query<(&mut TNavType, &ToRoot), (With<NavAsCurrentOrderInControl>, With<NavOrderFromControlViaRoot>)>,
    root_q: Query<&ToUnitControl>,
    control_q: Query<&ActiveOrderTerminal>
) {
    for (terminal, to_root) in nav_q.iter_mut() {
        nav_type_as_current_from_control_via_root(terminal, to_root, &root_q, &control_q);
    }
}

fn nav_type_as_current_from_control_via_root(
    mut terminal: Mut<TNavType>,
    to_root: &ToRoot,
    root_q: &Query<&ToUnitControl>,
    control_q: &Query<&ActiveOrderTerminal>
) {
    ref_caravan!(to_root::root_q(to_control) -> to_control::control_q(active_terminal););
    let Some(current) = active_terminal.current() else {
        terminal.0 = TypeId::of::<TNavType>();
        return;
    };
    terminal.0 = current;
}

/// TNavAttackTarget = NavAsCurrentOrderInControl + NavOrderFromControlViaRoot
pub fn nav_target_as_current_target_from_control_via_root_sys(
    mut nav_q: Query<(&mut TNavAttackTarget, &ToRoot), (With<NavAsCurrentOrderInControl>, With<NavOrderFromControlViaRoot>)>,
    root_q: Query<&ToUnitControl>,
    control_q: Query<&CurrentTarget>,
    target_q: Query<&GlobalTransform>
) {
    for (terminal, to_root) in nav_q.iter_mut() {
        nav_target_as_current_target_from_control_via_root(terminal, to_root, &root_q, &control_q, &target_q);
    }
}

fn nav_target_as_current_target_from_control_via_root(
    mut terminal: Mut<TNavAttackTarget>,
    to_root: &ToRoot,
    root_q: &Query<&ToUnitControl>,
    control_q: &Query<&CurrentTarget>,
    target_q: &Query<&GlobalTransform>
) {
    ref_caravan!(to_root::root_q(to_control) -> to_control::control_q(current_target););
    match current_target.read() {
        Some(target) => {
            let Ok(waypoint) = target_q.get(target) else {
                terminal.0 = Vec2::ZERO;
                return;
            };
            terminal.0 = waypoint.translation().truncate();
        },
        None => {
            terminal.0 = Vec2::ZERO;
        },
    }
    
}

/// TNavPureMove = NavAsCurrentOrderInControl + NavOrderFromControlViaRoot
pub fn nav_pure_move_as_current_from_control_via_root_sys(
    mut nav_q: Query<(&mut TNavPureMove, &ToRoot), (With<NavAsCurrentOrderInControl>, With<NavOrderFromControlViaRoot>)>,
    root_q: Query<&ToUnitControl>,
    control_q: Query<&TPureMoveOrders>
) {
    for (terminal, to_root) in nav_q.iter_mut() {
        nav_pure_move_as_current_from_control_via_root(terminal, to_root, &root_q, &control_q);
    }
}

fn nav_pure_move_as_current_from_control_via_root(
    mut terminal: Mut<TNavPureMove>,
    to_root: &ToRoot,
    root_q: &Query<&ToUnitControl>,
    control_q: &Query<&TPureMoveOrders>
) {
    ref_caravan!(to_root::root_q(to_control) -> to_control::control_q(pure_move_terminal););
    let Some(current) = pure_move_terminal.current() else {
        return;
    };
    terminal.0 = current.waypoint;
}

/// TNavAttackMove = NavAsCurrentOrderInControl + NavOrderFromControlViaRoot
pub fn nav_attack_move_as_current_from_control_via_root_sys(
    mut nav_q: Query<(&mut TNavAttackMove, &ToRoot), (With<NavAsCurrentOrderInControl>, With<NavOrderFromControlViaRoot>)>,
    root_q: Query<&ToUnitControl>,
    control_q: Query<&TAttackMoveOrders>
) {
    for (terminal, to_root) in nav_q.iter_mut() {
        nav_attack_move_as_current_from_control_via_root(terminal, to_root, &root_q, &control_q);
    }
}

fn nav_attack_move_as_current_from_control_via_root(
    mut terminal: Mut<TNavAttackMove>,
    to_root: &ToRoot,
    root_q: &Query<&ToUnitControl>,
    control_q: &Query<&TAttackMoveOrders>
) {
    ref_caravan!(to_root::root_q(to_control) -> to_control::control_q(attack_move_terminal););
    let Some(current) = attack_move_terminal.current() else {
        return;
    };
    terminal.0 = current.waypoint;
}