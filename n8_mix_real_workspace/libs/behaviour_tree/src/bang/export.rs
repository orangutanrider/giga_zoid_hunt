use bevy::prelude::*;

use ref_caravan::ref_caravan;

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
    mut node_q: Query<(&mut ExportPropagator, &Children), Changed<ExportPropagator>>,
    mut child_q: Query<&mut ExportPropagator>
) {
    for (mut propagator, children) in node_q.iter_mut() {
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
    child_q: &mut Query<&mut ExportPropagator>
) {
    let child = *child;
    ref_caravan!(@child::child_q(mut propagator););
    propagator.0 = true;
}