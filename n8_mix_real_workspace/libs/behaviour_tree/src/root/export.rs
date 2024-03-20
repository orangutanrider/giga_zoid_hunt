pub mod signal;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::BehaviourTreeExit;

use self::signal::ExportBang;

use super::*;

pub(crate) trait BehaviourTreeIntegrated: Component {
    fn bang(&mut self, v: bool);
}

pub(crate) trait RefBangExporter: Component {
    fn export(&mut self);
    fn activate(&mut self);
    fn reset(&mut self);
}
#[macro_export]
macro_rules! ref_bang_exporter {($t:ty) => {
    impl RefBangExporter for $t {
        fn export(&mut self) {
            self.export = true;
        }
        fn activate(&mut self) {
            self.bang = true;
        }
        fn reset(&mut self) {
            self.bang = false;
        }
    }
};}

// This could be replaced by the reset_behaviour_sys, having the RefBangExport require ResetBehaviour trait implementation.
// I decided to not do that though. It is the same either way, but this way keeps the two traits seperate, which should be more flexible.
/// Prefab system for resetting an exporter, whenever the behaviour tree updates.
/// (Inferred through reset bang on the root)
pub(crate) fn export_reset_sys<Exporter: RefBangExporter>(
    mut root_q: Query<(&mut Exporter, &ResetBang), Changed<ResetBang>>
) {
    for (mut export, reset) in root_q.iter_mut() {
        if !reset.is_active() {
            continue;
        }

        export.reset();
    }
}

/// Prefab system for telling the exporter to export, whenever the behaviour tree updates.
/// (Inferred through export bang on the root)
/// Does not export the bang value itself, it can only flag an exporter to export, other systems have to handle the exporting themselves.
pub(crate) fn export_bang_sys<Exporter: RefBangExporter>(
    mut root_q: Query<(&mut Exporter, &ExportBang), Changed<ExportBang>>
) {
    for (mut export, signal) in root_q.iter_mut() {
        if !signal.is_active() {
            continue;
        }

        export.export();
    }
}

#[derive(SystemParam)]
/// The exporting system is expected to check whatever value in the export component, that acts as the flag to export the bang.
/// Then lower the flag, once the bang has been exported.
pub(crate) struct ExportExitQuery<'w, 's, Exporter: Component>(
    pub Query<'w, 's, (&'static mut Exporter, &'static BehaviourTreeExit), (Changed<Exporter>, Changed<ExportBang>)>
);