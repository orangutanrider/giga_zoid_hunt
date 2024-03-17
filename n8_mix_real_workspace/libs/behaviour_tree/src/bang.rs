pub mod latch;
pub mod update;

use ref_caravan::ref_caravan;
use ref_paths::*;
use bevy::prelude::*;

use crate::{root::RootBang, ToBehaviourRoot};

#[derive(Component)]
/// Bang terminal
/// Holds the active/inactive state of a node
/// Sends internal changes to the root
pub(crate) struct TBang {
    active: bool,
    update_to_root: bool, // change to root
}
impl Default for TBang {
    fn default() -> Self {
        return Self::new()
    }
}
impl TBang { //! Constructor
    pub fn new() -> Self {
        return Self {
            active: false,
            update_to_root: false,
        }
    }
}

impl TBang { //! Set
    pub fn set_bang(&mut self, v: bool) {
        if self.active == v { return; }
        self.update_to_root = true;
        self.active = v;
    }
}
impl TBang { //! Get
    pub fn active(&self) -> bool {
        return self.active
    }
}

impl TBang { //! Internal
    fn propogate_bang(&mut self) {
        // deactivates without flagging a change
        self.active = false
    }
}

/// Deactivation propogation
fn bang_propogation_sys(
    node_q: Query<(&TBang, &Children), Changed<TBang>>,
    mut child_q: Query<&mut TBang>,
) {
    for (terminal, children) in node_q.iter() {
        if terminal.active() {
            continue;
        }

        for child in children.iter() {
            bang_propogation(&mut child_q, child);
        }
    }
}

fn bang_propogation(
    child_q: &mut Query<&mut TBang>,
    child: &Entity
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut terminal););

    terminal.propogate_bang();
}

fn bang_update_to_root_sys(
    mut node_q: Query<(&mut TBang, &ToBehaviourRoot), Changed<TBang>>,
    mut root_q: Query<&mut RootBang>
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
    root_q: &mut Query<&mut RootBang>,
) {
    ref_caravan!(to_root::root_q(mut root););

    root.update();
}