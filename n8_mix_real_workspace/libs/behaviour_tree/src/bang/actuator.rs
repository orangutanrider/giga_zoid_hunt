//! An actuator is a classification of component, that flags systems, that read the node's parent state, to activate or deactivate the local bang.

use bevy::{
    ecs::system::SystemParam, 
    prelude::*
};

use super::*;
use crate::{state::terminal::TState, ToParentNode};

#[derive(SystemParam)]
/// Standard query set for bang actuator systems
pub struct ActuatorQueries<'w, 's, Actuator: Component> {
    pub node_q: Query<'w, 's, (&'static mut Bang, &'static ActuatorPropagator, &'static ToParentNode), (With<Actuator>, Changed<ActuatorPropagator>)>,
    pub parent_q:  Query<'w, 's, &'static TState>,
}

/// Prefab system for bang actuators that are flagged by a single component
pub fn bang_actuator_sys<F, Actuator: Component>(
    actuator_qs: ActuatorQueries<Actuator>,
    actuator_logic: F
) where F: Fn(&TState) -> bool {
    let mut node_q = actuator_qs.node_q;
    let parent_q = &actuator_qs.parent_q;
    
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        actuator_to_bang(local_bang, to_parent, parent_q, &actuator_logic)
    }
}

/// Prefab function for bang actuator systems
pub fn actuator_to_bang<F>(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
    actuator_logic: F
) where F: Fn(&TState) -> bool { 
    ref_caravan!(to_parent::parent_q((parent_state)););
    local_bang.actuator_set(actuator_logic(parent_state));
}

#[derive(Component)]
pub struct ActuatorPropagator(bool);
impl Default for ActuatorPropagator {
    fn default() -> Self {
        return Self::new()
    }
}
impl ActuatorPropagator { 
    pub fn new() -> Self {
        return Self(false)
    }

    fn is_propagating(&self) -> bool {
        return self.0
    }
}

/// PreUpdate System.
/// Propogates from changed TState(s) to ActuatorPropagator(s).
/// It also checks the bang, before propagating (inactive will not propagate).
/// The ActuatorPropagator signals actuators to set their Bang. 
/// (If they activate their bang, it'll cause that bang to propogate to child ActuatorPropagator(s)).
pub fn state_to_actuator_propagation_sys(
    mut node_q: Query<(&TState, &Bang, &Children), Changed<TState>>,
    mut child_q: Query<&mut ActuatorPropagator>
) {
    for (state, bang, children) in node_q.iter_mut() {
        if !bang.is_active() || !state.changed() {
            continue;
        }

        for child in children.iter() {
            actuator_propagation(child, &mut child_q);
        }
    }
}

/// PreUpdate System.
/// Propogates from activated Bang(s) to ActuatorPropagator(s)
/// The ActuatorPropagator signals actuators to set their Bang. 
/// (If they activate their bang, it'll cause that bang to propogate to child ActuatorPropagator(s))
pub fn bang_to_actuator_propagation_sys(
    mut node_q: Query<(&Bang, &Children), Changed<Bang>>,
    mut child_q: Query<&mut ActuatorPropagator>
) {
    for (bang, children) in node_q.iter_mut() {
        if !bang.is_active() {
            continue;
        }

        for child in children.iter() {
            actuator_propagation(child, &mut child_q);
        }
    }
}

fn actuator_propagation(
    child: &Entity,
    child_q: &mut Query<&mut ActuatorPropagator>
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut propagator););
    propagator.0 = true;
}

/// PostUpdate system.
/// Deactivates active propagators.
pub fn end_actuator_propagation_sys(
    mut node_q: Query<&mut ActuatorPropagator, Changed<ActuatorPropagator>>,
) {
    for mut actuator in node_q.iter_mut() {
        actuator.bypass_change_detection();
        actuator.0 = false;
    }
}