use bevy::{
    ecs::system::SystemParam, 
    prelude::*
};

use super::*;
use crate::{state::terminal::TState, ToParentNode};

// A bang latch is a classification of component.
// It will flag system(s) that can activate the local bang terminal.
// The systems are expected to do this via reading the parent node's state, and deciding using their own logic.
// They are not expected to switch on their latch, if the parent's bang is inactive.
// They only need to run when their parent node changes, which is inferred through the latch propagator component.

#[derive(SystemParam)]
/// Standard query set for bang latch systems
pub(crate) struct LatchQueries<'w, 's, Latch: Component> {
    pub node_q: Query<'w, 's, (&'static mut TBang, &'static LatchPropagator, &'static ToParentNode), (With<Latch>, Changed<LatchPropagator>)>,
    pub parent_q:  Query<'w, 's, (&'static TState, &'static TBang)>,
}

/// Prefab system for bang latches that are flagged by a single component
pub(crate) fn bang_latch_sys<F, Latch: Component>(
    mut latch_qs: LatchQueries<Latch>,
    latch_logic: F
) where F: Fn(&TState, &TBang) -> bool { 
    let mut node_q = latch_qs.node_q;
    let parent_q = &latch_qs.parent_q;

    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        latch_set_bang(local_bang, propagator, to_parent, parent_q, &latch_logic)
    }
}

/// Prefab function for bang latch systems
pub(crate) fn latch_set_bang<F>(
    mut local_bang: Mut<TBang>,
    propagator: &LatchPropagator,
    to_parent: &ToParentNode,
    parent_q: &Query<(&TState, &TBang)>,
    latch_logic: F
) where F: Fn(&TState, &TBang) -> bool { 
    ref_caravan!(to_parent::parent_q((parent_state, parent_bang)););
    
    // (Latches should not attempt activation, when the parent node is not active.)
    // (Latches only need to update, when their parent node has changed.)
    if !parent_bang.active() || !propagator.propagating() {
        return;
    }

    local_bang.set_bang(latch_logic(parent_state, parent_bang));
}

#[derive(Component)]
/// A basic latch will activate its node, if the parent's bang is active.
/// It does not care about the parent's state.
pub struct BasicLatch;

fn basic_latch_sys(
    mut node_q: Query<(&mut TBang, &LatchPropagator, &ToParentNode), (With<BasicLatch>, Changed<LatchPropagator>)>,
    parent_q: Query<&TBang>,
) {
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        basic_latch_set_bang(local_bang, propagator, to_parent, &parent_q)
    }
}

fn basic_latch_set_bang(
    mut local_bang: Mut<TBang>,
    propagator: &LatchPropagator,
    to_parent: &ToParentNode,
    parent_q: &Query<&TBang>,
) {
    ref_caravan!(to_parent::parent_q(parent_bang););

    // (Latches should not attempt activation, when the parent node is not active.)
    // (Latches only need to update, when their parent node has changed.)
    if !parent_bang.active() || !propagator.propagating() {
        return;
    }

    local_bang.set_bang(true);
}

#[derive(Component)]
/// When a bang terminal is changed to true, the  latch propagators, on the children of that entity, get activated, by a system.
/// When a latch propagator is activated, the latch systems can check to see if their bang terminal should be activated now.
/// (And activate it, if it should.) (Restarting the cycle of propagation.)
pub(crate) struct LatchPropagator(bool);
impl Default for LatchPropagator {
    fn default() -> Self {
        return Self::new()
    }
}
impl LatchPropagator { 
    pub fn new() -> Self {
        return Self(false)
    }

    fn propagating(&self) -> bool {
        return self.0
    }
}

fn latch_propagation_sys(
    mut node_q: Query<(&TBang, &Children, &mut LatchPropagator), Changed<TBang>>,
    mut child_q: Query<&mut LatchPropagator>
) {
    for (terminal, children, mut propagator) in node_q.iter_mut() {
        if !terminal.active() {
            continue;
        }

        for child in children.iter() {
            latch_propagation(child, &mut child_q);
        }
        propagator.0 = false;
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