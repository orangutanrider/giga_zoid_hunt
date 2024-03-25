//! A release is a classification of component that flags systems to disengage its local bang, based on parent state.

use bevy::{
    ecs::system::SystemParam, 
    prelude::*
};

use super::*;
use crate::{state::terminal::TState, ToParentNode};

#[derive(SystemParam)]
/// Standard query set for bang release systems
pub struct ReleaseQueries<'w, 's, Release: Component> {
    pub node_q: Query<'w, 's, (&'static mut Bang, &'static ReleasePropagator, &'static ToParentNode), (With<Release>, Changed<ReleasePropagator>)>,
    pub parent_q:  Query<'w, 's, &'static TState>,
}

/// Prefab system for bang releases that are flagged by a single component
pub fn bang_release_sys<F, Release: Component>(
    release_qs: ReleaseQueries<Release>,
    release_logic: F
) where F: Fn(&TState) -> bool {
    let mut node_q = release_qs.node_q;
    let parent_q = &release_qs.parent_q;
    
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        release_to_bang(local_bang, to_parent, parent_q, &release_logic)
    }
}

/// Prefab function for bang release systems
pub fn release_to_bang<F>(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
    release_logic: F
) where F: Fn(&TState) -> bool { 
    ref_caravan!(to_parent::parent_q((parent_state)););

    if !release_logic(parent_state) {
        return;
    }

    local_bang.release_deactivate();
}

#[derive(Component)]
pub struct ReleasePropagator(bool);
impl Default for ReleasePropagator {
    fn default() -> Self {
        return Self::new()
    }
}
impl ReleasePropagator { 
    pub fn new() -> Self {
        return Self(false)
    }

    fn is_propagating(&self) -> bool {
        return self.0
    }
}

/// If state change and bang active, propagate to children
pub fn release_propagation_sys(
    mut node_q: Query<(&TState, &Bang, &Children), Changed<TState>>,
    mut child_q: Query<&mut ReleasePropagator>
) {
    for (state, bang, children) in node_q.iter_mut() {
        if !bang.is_active() || !state.changed() {
            continue;
        }

        for child in children.iter() {
            release_propagation(child, &mut child_q);
        }
    }
}

fn release_propagation(
    child: &Entity,
    child_q: &mut Query<&mut ReleasePropagator>
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut propagator););
    propagator.0 = true;
}

/// PostUpdate system.
/// Deactivates active propagators.
pub fn end_release_propagation_sys(
    mut node_q: Query<&mut ReleasePropagator, Changed<ReleasePropagator>>,
) {
    for mut propagator in node_q.iter_mut() {
        propagator.bypass_change_detection();
        propagator.0 = false;
    }
}