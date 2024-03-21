//! A RefBang is a classification of component, that as of recieving a bang signal, will send that bang value to be exported in the root.
//! In the root, the export is recieved by a matching component, which waits to recieve an export signal, to export its recieved bang state.

use bevy::prelude::*;

use ref_caravan::ref_caravan;
use ref_paths::*;
use crate::{root::export::RefBangExporter, ToBehaviourRoot};

use super::Bang;

/// Prefab system.
/// This will export a bang value to its exporter, when the propogation wave has reached that ref-bang.
pub fn ref_bang_to_export_sys<RefBang: Component, Exporter: RefBangExporter>(
    node_q: Query<(&ToBehaviourRoot, &ExportPropagator), (Changed<ExportPropagator>, With<RefBang>)>,
    mut root_q: Query<&mut Exporter>
) {
    for (to_root, propagator) in node_q.iter() {
        if !propagator.is_propagating() {
            continue;
        }
        ref_bang_to_export(to_root, &mut root_q);
    }
}

/// Prefab function, used with the ref_bang_to_export_sys.
pub fn ref_bang_to_export<Export: RefBangExporter>(
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<&mut Export>
) {
    ref_caravan!(to_root::root_q(mut export););
    export.activate();
}

#[derive(Component)]
pub struct ExportPropagator(bool);
impl Default for ExportPropagator {
    fn default() -> Self {
        return Self::new()
    }
}
impl ExportPropagator { 
    pub fn new() -> Self {
        return Self(false)
    }

    fn is_propagating(&self) -> bool {
        return self.0
    }
}

/// Handles the propogation of ExportPropagator(s) to child ExportPropagator(s)
/// When a tree reset happens, it causes the propogation out from the root.
/// Each propogator propogates to children with activated bangs.
/// They signal to the bang references, to send their reference to the exporter at the root.
pub fn export_propogation_sys(
    mut node_q: Query<(&mut ExportPropagator, &Children), Changed<ExportPropagator>>,
    mut child_q: Query<(&mut ExportPropagator, &Bang)>
) {
    for (mut propagator, children) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        propagator.bypass_change_detection();
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