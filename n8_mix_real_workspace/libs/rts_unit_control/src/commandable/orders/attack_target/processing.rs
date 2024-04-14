use bevy::prelude::*;

use crate::validate_active_terminal_c;

use super::*;

#[derive(Component, Default)]
pub struct UntilTargetGoneProcessor;

#[derive(Component)]
// This component does not ever get cleared by the way.
// That is a ToDo issue, but it is non-critical, so it is un-dealt with.
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

    pub fn read(&self) -> &HashSet<Entity> {
        return &self.0
    }

    pub fn new_insert(targeter: Entity) -> Self {
        let mut new_self = Self(HashSet::new());
        new_self.insert_targeter(targeter);
        return new_self
    }

    pub fn insert_targeter(&mut self, targeter: Entity) {
        self.0.insert(targeter);
    }

    pub fn remove_targeter(&mut self, targeter: &Entity) {
        self.0.remove(&targeter);
    }
}

pub fn current_target_clear_sys(
    mut control_q: Query<(&mut TCurrentTarget, &mut OrderProcessedAgar), Changed<ClearOrdersBang>>,
) {
    for (mut target_holder, mut agar) in control_q.iter_mut() {
        target_holder.0 = None;
        agar.bang();
    }
}

pub fn current_target_validation_sys(
    mut control_q: Query<(Entity, &mut TCurrentTarget, &mut OrderProcessedAgar)>,
    target_q: Query<&TargetedBy>,
) {
    for (control, mut held_target, mut agar) in control_q.iter_mut() {
        let Some(target) = held_target.0 else {
            continue;
        };

        let Ok(targeters) = target_q.get(target) else {
            held_target.0 = None;
            agar.bang();
            continue;
        };
        if !targeters.0.contains(&control) { 
            held_target.0 = None;
            agar.bang();
            continue;
        }
    } 
} 

// If the functionality, for the OrderProcessedAgar being an optional component, is wanted, then split this into two systems that're disjointed via query filtering.
/// Move pre-prepared attack target orders to the current target component.
pub fn target_to_current_sys (
    mut control_q: Query<
        (Entity, &mut TAttackTargetOrders, &mut TCurrentTarget, &mut TActiveOrderType, &mut OrderProcessedAgar), 
        (With<UntilTargetGoneProcessor>, Or<(Changed<TActiveOrderType>, Changed<TCurrentTarget>)>)
    >,
    target_q: Query<&TargetedBy>,
) {
    for (control, mut orders, mut current_target, mut active_types, mut agar) in control_q.iter_mut() {
        validate_active_terminal_c!(TAttackTargetOrders, active_types);

        if current_target.is_some() { 
            // You could expect it to be nothing, since it has changed.
            // I didn't do that though, cause this allows other things to manipulate their current target.
            continue;
        }

        // Get new current target from stack
        let Some(new_target) = orders.move_current() else { // If no new target
            active_types.clear_current(); // Stop processing target orders.
            agar.bang(); // An order has been processed!
            continue;
        };

        // Validate if target still exists, this is done by checking for the "TargetedBy" component.
        let Ok(targeters) = target_q.get(new_target.target) else { // If entity does not contain a "TargetedBy" component.
            // If it does not have that component, it is assumed that it is a different than the one originally targeted.
            active_types.clear_current(); // Stop processing target orders.
            agar.bang(); // An order has been processed!
            continue;
        };

        // Validate target again, it may have the "TargetedBy" component, but is it still the correct entity?
        if !targeters.0.contains(&control) { // If its "TargetedBy" component does not contain a entry of our entity.
            // Then it is assumed that this is a different entity than before.
            active_types.clear_current(); // Stop processing target orders.
            agar.bang(); // An order has been processed!
            continue;
        }

        // If the order is still valid, move it to the "CurrentTarget" component.
        // (The order is moved out of the order stack earlier, when the order stack is first accessed.)
        current_target.bypass_change_detection();
        current_target.0 = Some(new_target.target); 
    }
}

/* 
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
*/

/* 
pub fn abort_current_target_bang_sys(
    mut q: Query<&mut AbortCurrentTargetBang, Changed<AbortCurrentTargetBang>>,
) {
    for mut bang in q.iter_mut() {
        bang.bypass_change_detection();
        bang.0 = false;
    }
}
*/

/* 
pub fn abort_current_target_sys(
    mut control_q: Query<(Entity, &mut TCurrentTarget, &mut OrderProcessedAgar), Changed<AbortCurrentTargetBang>>,
    mut target_q: Query<&mut TargetedBy>,
) {
    for (control, mut target_holder, mut agar) in control_q.iter_mut() {
        let Some(target) = target_holder.0 else {
            continue; // It isn't expected that this could happen, but it is fine if it does.
        };

        let Ok(mut targeters) = target_q.get_mut(target) else {
            continue; // Same with this, but it's expected be rare instead of impossible.
        };

        target_holder.0 = None;
        targeters.0.remove(&control);
        agar.bang();
    }
}
*/