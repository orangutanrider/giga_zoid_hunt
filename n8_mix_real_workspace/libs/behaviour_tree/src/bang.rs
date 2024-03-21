pub mod actuator;
pub mod latch;
pub mod release;

pub mod reference;
pub mod fizzler;

use ref_caravan::ref_caravan;
use ref_paths::*;
use bevy::prelude::*;

use crate::root::reset::ResetBang;
use crate::ToBehaviourRoot;
use self::latch::*;
use self::reference::*;
use self::actuator::*;
use self::release::*;

pub struct BangPlugin;
impl Plugin for BangPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            // actuator
            state_to_actuator_propagation_sys,
            bang_to_actuator_propagation_sys,
            // latch
            state_to_latch_propagation_sys,
            bang_to_latch_propagation_sys,
        ));
        app.add_systems(Update,(
            basic_latch_sys, // latch
            export_propogation_sys, // reference
            release_propagation_sys, // release
            bang_update_to_root_sys, // bang
        ));
        app.add_systems(PostUpdate, (
            end_actuator_propagation_sys, // actuator
            end_latch_propagation_sys, // latch
            end_release_propagation_sys, // release
            deactivation_propagation_sys // bang
        ));
    }
}

#[derive(Component)]
/// Bang
/// Holds the active/inactive state of a node
/// Sends internal changes to the root
pub struct Bang {
    active: bool,
    update_to_root: bool, // Causes a tree reset (re-exporting all bang references)
}
impl Default for Bang {
    fn default() -> Self {
        return Self::new(false)
    }
}
impl Bang { //! Constructor
    pub fn new(bang: bool) -> Self {
        return Self {
            active: bang,
            update_to_root: bang,
        }
    }
}

impl Bang { //! Set
    /// An actuator is a classification of component, that activates or deactivates a Bang based on the parent node's state.
    /// It does not need to check if the parent Bang is active, if it uses the LatchPropagator.
    /// (As a latch propagator, will only propagate if the parent is active).
    pub fn actuator_set(&mut self, bang: bool) {
        if self.active == bang { return; }
        self.update_to_root = true;
        self.active = bang;
    }

    /// A latch is a classification of component, that activates a Bang based on the parent node's state.
    /// It does not need to check if the parent Bang is active, if it uses the LatchPropagator.
    /// (As a latch propagator, will only propagate if the parent is active).
    pub fn latch_activate(&mut self) {
        if self.active == true { return; }
        self.update_to_root = true;
        self.active = true;
    }

    /// A release is a classification of component, that deactivates a Bang based on the parent node's state.
    pub fn release_deactivate(&mut self) {
        if self.active == true { return; }
        self.update_to_root = true;
        self.active = true;
    }

    /// A fizzler is a classification of component, that deactivates a Bang based on local node state.
    pub fn fizzler_deactivate(&mut self) {
        if self.active == false { return; }
        self.update_to_root = true;
        self.active = false;
    }
}
impl Bang { //! Get
    pub fn is_active(&self) -> bool {
        return self.active
    }
}

impl Bang { //! Internal
    pub(crate) fn from_root(&mut self, bang: bool) {
        // Sets without flagging a change
        self.active = bang;
    }

    fn propagate_deactivation(&mut self) {
        // Deactivates without flagging a change
        self.active = false
    }
}

/// Will propogate any deactivated Bang, to deactivate its children.
pub fn deactivation_propagation_sys(
    node_q: Query<(&Bang, &Children), Changed<Bang>>,
    mut child_q: Query<&mut Bang>,
) {
    for (bang, children) in node_q.iter() {
        if bang.is_active() {
            continue;
        }

        for child in children.iter() {
            deactivation_propagation(&mut child_q, child);
        }
    }
}

fn deactivation_propagation(
    child_q: &mut Query<&mut Bang>,
    child: &Entity
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut bang););

    bang.propagate_deactivation();
}

/// When a bang is flagged as updated, this system will lower the flag and send the signal to root
/// (Causing a reset)
pub fn bang_update_to_root_sys(
    mut node_q: Query<(&mut Bang, &ToBehaviourRoot), Changed<Bang>>,
    mut root_q: Query<&mut ResetBang>
) {
    for (mut bang, to_root) in node_q.iter_mut() {
        if !bang.update_to_root {
            continue;
        }
        bang.bypass_change_detection();
        bang.update_to_root = false;

        bang_update_to_root(to_root, &mut root_q);
    }
}

fn bang_update_to_root(
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<&mut ResetBang>,
) {
    ref_caravan!(to_root::root_q(mut reset););

    reset.bang();
}