pub mod latch;
pub mod export;

use ref_caravan::ref_caravan;
use ref_paths::*;
use bevy::prelude::*;

use crate::{root::ResetBang, ToBehaviourRoot};

#[derive(Component)]
/// Bang terminal
/// Holds the active/inactive state of a node
/// Sends internal changes to the root
pub(crate) struct Bang {
    active: bool,
    update_to_root: bool, // change to root
}
impl Default for Bang {
    fn default() -> Self {
        return Self::new()
    }
}
impl Bang { //! Constructor
    pub fn new() -> Self {
        return Self {
            active: false,
            update_to_root: false,
        }
    }
}

impl Bang { //! Set
    /// Bang activation, should only be done by latches, that are doing via reading the parent node
    pub fn activate(&mut self) {
        if self.active == true { return; }
        self.update_to_root = true;
        self.active = true;
    }

    /// Bang decativation, should only be done internally, by behaviour managing systems, that do not read beyond their node.
    /// They should only execute, when their bang is active.
    pub fn deactivate(&mut self) {
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
    fn propogate_bang(&mut self) {
        // deactivates without flagging a change
        self.active = false
    }
}

/// Deactivation propogation
fn bang_propogation_sys(
    node_q: Query<(&Bang, &Children), Changed<Bang>>,
    mut child_q: Query<&mut Bang>,
) {
    for (terminal, children) in node_q.iter() {
        if terminal.is_active() {
            continue;
        }

        for child in children.iter() {
            bang_propogation(&mut child_q, child);
        }
    }
}

fn bang_propogation(
    child_q: &mut Query<&mut Bang>,
    child: &Entity
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut terminal););

    terminal.propogate_bang();
}

fn bang_update_to_root_sys(
    mut node_q: Query<(&mut Bang, &ToBehaviourRoot), Changed<Bang>>,
    mut root_q: Query<&mut ResetBang>
) {
    for (mut terminal, to_root) in node_q.iter_mut() {
        if !terminal.update_to_root {
            continue;
        }
        terminal.update_to_root = false;

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