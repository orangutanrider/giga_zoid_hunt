use bevy::{ prelude::*};

use ref_caravan::ref_caravan;
use ref_paths::*;
use crate::ToBehaviourRoot;

use super::Bang;

pub(crate) trait RefBangExport {

}

pub(crate) fn ref_bang_export_sys<RefBang: Component, RefBangExport: Component>(
    node_q: Query<&ToBehaviourRoot, (Changed<ExportPropagator>, With<RefBang>)>,
    mut root_q: Query<&mut RefBangExport>
) {
    for to_root in node_q.iter() {
        ref_bang_export(to_root, &mut root_q)
    }
}

pub(crate) fn ref_bang_export<RefBangExport: Component>(
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<&mut RefBangExport>
) {
    ref_caravan!(to_root::root_q(export););
    
}

#[derive(Component)]
pub(crate) struct ExportPropagator(bool);
impl Default for ExportPropagator {
    fn default() -> Self {
        return Self::new()
    }
}
impl ExportPropagator { 
    pub fn new() -> Self {
        return Self(false)
    }

    fn propagating(&self) -> bool {
        return self.0
    }
}

fn export_propogation_sys(
    mut node_q: Query<(&Bang, &mut ExportPropagator, &Children), Changed<ExportPropagator>>,
    mut child_q: Query<(&mut ExportPropagator, &Bang)>
) {
    for (bang, mut propagator, children) in node_q.iter_mut() {
        if !propagator.propagating() {
            continue;
        }
        propagator.0 = false;

        for child in children.iter() {
            export_propogation(child, &mut child_q);
        }
    }
}

fn export_propogation(
    child: &Entity,
    child_q: &mut Query<(&mut ExportPropagator, &Bang)>
) {
    let child = *child;
    ref_caravan!(@child::child_q((mut propagator, bang)););

    if !bang.is_active() {
        return;
    }
    propagator.0 = true;
}