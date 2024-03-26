use bevy::prelude::*;

use crate::validate_active_terminal_c;

use super::*;

// TODO, respond to death, to wipe self from the targeters.
// Plan for this is a death-terminal bang thing, and the signal will be sent here (control).

pub fn abort_current_target_sys(
    mut control_q: Query<(Entity, &mut CurrentTarget), Changed<AbortCurrentTargetBang>>,
    mut target_q: Query<&mut TargetedBy>,
) {
    for (control, mut target_holder) in control_q.iter_mut() {
        let Some(target) = target_holder.0 else {
            continue; // It isn't expected that this could happen, but it is fine if it does.
        };

        let Ok(mut targeters) = target_q.get_mut(target) else {
            continue; // Same with this, but it's expected be rare instead of impossible.
        };

        target_holder.0 = None;
        targeters.0.remove(&control);
    }
}

pub fn current_target_validation_sys(
    mut control_q: Query<(Entity, &mut CurrentTarget)>,
    target_q: Query<&TargetedBy>,
) {
    for (control, mut held_target) in control_q.iter_mut() {
        let Some(target) = held_target.0 else {
            continue;
        };

        let Ok(targeters) = target_q.get(target) else {
            held_target.0 = None;
            continue;
        };
        if !targeters.0.contains(&control) { 
            held_target.0 = None;
            continue;
        }
    } 
} 

pub fn target_to_current_sys (
    mut control_q: Query<(Entity, &mut TAttackTargetOrders, &mut CurrentTarget, &mut ActiveOrderTerminal), Or<(Changed<ActiveOrderTerminal>, Changed<CurrentTarget>)>>,
    target_q: Query<&TargetedBy>,
) {
    for (control, mut orders, mut current_target, mut active_types) in control_q.iter_mut() {
        validate_active_terminal_c!(TAttackTargetOrders, active_types);

        if current_target.is_some() { 
            // You could expect it to be nothing, since it has changed.
            // I didn't do that though, cause this allows other things to manipulate their current target.
            continue;
        }

        // Get new current target from stack
        let Some(new_target) = orders.move_current() else {
            active_types.clear_current();
            continue;
        };

        // Validate if target still exists
        let Ok(targeters) = target_q.get(new_target.target) else {
            active_types.clear_current();
            continue;
        };
        // Validate if the target is the same.
        // NOT: If it's a new entity (with this component by-chance), or the target has interfeered with the data itself.
        if !targeters.0.contains(&control) { 
            active_types.clear_current();
            continue;
        }

        // Move target to current target component, bypassing change detection.
        current_target.bypass_change_detection();
        current_target.0 = Some(new_target.target);
    }
}

pub fn abort_current_target_bang_sys(
    mut q: Query<&mut AbortCurrentTargetBang, Changed<AbortCurrentTargetBang>>,
) {
    for mut bang in q.iter_mut() {
        bang.bypass_change_detection();
        bang.0 = false;
    }
}

#[derive(Component)]
pub struct AbortCurrentTargetBang(bool);
impl Default for AbortCurrentTargetBang {
    fn default() -> Self {
        Self(false)
    }
}
impl AbortCurrentTargetBang {
    pub fn new() -> Self {
        return Self(false)
    }
}

#[derive(Component)]
pub struct CurrentTarget(Option<Entity>);
impl Default for CurrentTarget {
    fn default() -> Self {
        Self(None)
    }
}
impl CurrentTarget {
    pub fn new() -> Self {
        return Self(None)
    }

    pub fn is_some(&self) -> bool {
        return match self.0 {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Component)]
pub struct TargetedBy(HashSet<Entity>);
impl Default for TargetedBy {
    fn default() -> Self {
        Self(HashSet::new())
    }
}
impl TargetedBy {
    pub fn new() -> Self {
        return Self(HashSet::new())
    }
}