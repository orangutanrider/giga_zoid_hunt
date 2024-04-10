use bevy::prelude::*;

use ref_caravan::ref_caravan;

use crate::bang::Bang;
use super::reset::ResetBang;

#[derive(Component)]
/// Bang for the entire behaviour tree
/// Deciding if it is active or not
pub struct RootBang{
    active: bool,
    changed: bool
}
impl Default for RootBang {
    fn default() -> Self {
        return Self::new(true)
    }
}
impl RootBang { //! Constructor
    pub fn new(bang: bool) -> Self {
        return Self{
            active: bang,
            changed: true,
        }
    }
}

impl RootBang { //! Set
    pub fn set(&mut self, bang: bool) {
        if self.active == bang { return; }
        self.changed = true;
        self.active = bang;
    }
}
impl RootBang { //! Get
    pub fn is_active(&self) -> bool {
        return self.active
    }
}

pub fn propagate_spawned_root_bang_sys(
    mut root_q: Query<(&mut RootBang, &mut ResetBang, &Children), Added<RootBang>>,
    mut child_q: Query<&mut Bang>,
) {
    for (mut root_bang, mut reset_bang, children) in root_q.iter_mut() {
        if !root_bang.changed {
            continue;
        }
        root_bang.bypass_change_detection();
        root_bang.changed = false;

        reset_bang.bang();

        for child in children.iter() {
            propagate_root_bang(root_bang.is_active(), child, &mut child_q)
        }
    }
}

pub fn propagate_root_bang_sys(
    mut root_q: Query<(&mut RootBang, &mut ResetBang, &Children), Changed<RootBang>>,
    mut child_q: Query<&mut Bang>,
) {
    for (mut root_bang, mut reset_bang, children) in root_q.iter_mut() {
        if !root_bang.changed {
            continue;
        }
        root_bang.bypass_change_detection();
        root_bang.changed = false;

        reset_bang.bang();

        for child in children.iter() {
            propagate_root_bang(root_bang.is_active(), child, &mut child_q)
        }
    }
}

fn propagate_root_bang(
    root_bang: bool,
    child: &Entity,
    child_q: &mut Query<&mut Bang>,
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut bang););

    bang.from_root(root_bang);
}