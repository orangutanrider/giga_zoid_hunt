use bevy::prelude::*;

use ref_caravan::ref_caravan;

use crate::ToBehaviourRoot;

use super::Bang;

fn ref_bang_export_sys<RefBang: Component, RefBangExport: Component>(
    node_q: Query<&ToBehaviourRoot, (Changed<ExportPropagator>, With<RefBang>)>,
    root_q: Query<&mut RefBangExport>
) {

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