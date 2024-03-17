use bevy::{
    ecs::system::SystemParam, 
    prelude::*
};

use super::*;
use crate::{state::terminal::TState, ToParentNode};

/// A bang latch is a classification of component.
/// It will flag system(s) that can activate the local bang terminal.
/// The trait provides a blueprint for doing so.
/// (Via reading the parent node's state, and deciding, using its own written logic.)
/// (Latches should not attempt activation, when the parent node is not active.)
/// (Latches only need to update, when their parent node has changed.)
pub(crate) trait BangLatch {
    fn set_bang(
        mut bang: Mut<TBang>,
        parent_state: &TState,
        parent_bang: &TBang,
        propogator: &LatchPropagator,
    ) {
        // (Latches should not attempt activation, when the parent node is not active.)
        // (Latches only need to update, when their parent node has changed.)
        if !parent_bang.active() || !propogator.propagating() {
            return;
        }

        bang.set_bang(Self::latch_logic(parent_state, parent_bang));
    }

    fn latch_logic(
        parent_state: &TState,
        parent_bang: &TBang,
    ) -> bool; // (Via reading the parent node's state, and deciding, using its own written logic.)
}

#[derive(SystemParam)]
/// Standard query set for bang latch systems
pub struct LatchQueries<'w, 's, Latch: Component> {
    node_q: Query<'w, 's, (&'static TBang, &'static LatchPropagator, &'static ToParentNode), (With<Latch>, Changed<LatchPropagator>)>,
    parent_q:  Query<'w, 's, &'static mut TState>,
}

#[derive(Component)]
/// When a bang terminal is changed to true, the child latch propagators get activated, by a system.
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

fn latch_propogation_sys(
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