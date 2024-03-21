//! A bang latch is a classification of component.
//! It will flag system(s) that can activate the local bang terminal.
//! The systems are expected to do this via reading the parent node's state, and deciding using their own logic.
//! They are not expected to switch on their latch, if the parent's bang is inactive.
//! They only need to run when their parent node changes, which is inferred through the latch propagator component.

use bevy::{
    ecs::system::SystemParam, 
    prelude::*
};

use super::*;
use crate::{state::terminal::TState, ToParentNode};

#[derive(SystemParam)]
/// Standard query set for bang latch systems
pub struct LatchQueries<'w, 's, Latch: Component> {
    pub node_q: Query<'w, 's, (&'static mut Bang, &'static LatchPropagator, &'static ToParentNode), (With<Latch>, Changed<LatchPropagator>)>,
    pub parent_q:  Query<'w, 's, &'static TState>,
}

/// Prefab system for bang latches that are flagged by a single component
pub fn bang_latch_sys<F, Latch: Component>(
    latch_qs: LatchQueries<Latch>,
    latch_logic: F
) where F: Fn(&TState) -> bool { 
    let mut node_q = latch_qs.node_q;
    let parent_q = &latch_qs.parent_q;

    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        latch_set_bang(local_bang, to_parent, parent_q, &latch_logic)
    }
}

/// Prefab function for bang latch systems
pub fn latch_set_bang<F>(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
    latch_logic: F
) where F: Fn(&TState) -> bool { 
    ref_caravan!(to_parent::parent_q((parent_state)););
    
    if !latch_logic(parent_state) {
        return;
    }

    local_bang.latch_activate();
}

#[derive(Component)]
/// A basic latch will activate its node, if the parent's bang is active.
/// It does not care about the parent's state.
pub struct BasicLatch;

/// On components that have a basic latch, this system will activate the bang of that node.
/// If the parent's bang is active.
pub fn basic_latch_sys(
    mut node_q: Query<(&mut Bang, &LatchPropagator), (With<BasicLatch>, Changed<LatchPropagator>)>,
) {
    for (mut local_bang, propagator) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        local_bang.latch_activate();
    }
}

#[derive(Component)]
/// When a bang terminal is changed to true, the  latch propagators, on the children of that entity, get activated, by a system.
/// When a latch propagator is activated, the latch systems can check to see if their bang terminal should be activated now.
/// (And activate it, if it should.) (Restarting the cycle of propagation.)
pub struct LatchPropagator(bool);
impl Default for LatchPropagator {
    fn default() -> Self {
        return Self::new()
    }
}
impl LatchPropagator { 
    pub fn new() -> Self {
        return Self(false)
    }

    fn is_propagating(&self) -> bool {
        return self.0
    }
}

/// Propogates from changed TState(s) to LatchPropagator(s).
/// It also checks the bang, before propagating (inactive will not propagate).
/// The LatchPropagator signals latches to check if they can activate their Bang. 
/// (If they activate their bang, it'll cause that bang to propogate to child LatchPropagator(s)).
pub fn state_to_latch_propagation_sys(
    mut node_q: Query<(&TState, &Bang, &Children), Changed<TState>>,
    mut child_q: Query<&mut LatchPropagator>
) {
    for (state, bang, children) in node_q.iter_mut() {
        if !bang.is_active() || !state.changed() {
            continue;
        }

        for child in children.iter() {
            latch_propagation(child, &mut child_q);
        }
    }
}

/// Propogates from activated Bang(s) to LatchPropagator(s)
/// The LatchPropagator signals latches to check if they can activate their Bang. 
/// (If they activate their bang, it'll cause that bang to propogate to child LatchPropagator(s))
pub fn bang_to_latch_propagation_sys(
    mut node_q: Query<(&Bang, &Children), Changed<Bang>>,
    mut child_q: Query<&mut LatchPropagator>
) {
    for (bang, children) in node_q.iter_mut() {
        if !bang.is_active() {
            continue;
        }

        for child in children.iter() {
            latch_propagation(child, &mut child_q);
        }
    }
}

fn latch_propagation(
    child: &Entity,
    child_q: &mut Query<&mut LatchPropagator>
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut propagator););
    propagator.0 = true;
}

/// PostUpdate system
pub fn end_latch_propagation_sys(
    mut node_q: Query<&mut LatchPropagator, Changed<LatchPropagator>>,
) {
    for mut latch in node_q.iter_mut() {
        if !latch.is_propagating() {
            continue;
        }
        latch.0 = false;
    }
}