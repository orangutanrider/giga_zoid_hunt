//! Core bundles

use bevy::prelude::*;

use crate::{
    *,
    state::output::StateOutput,
    state::terminal::TState,
    bang::{
        Bang,
        latch::LatchPropagator,
        reference::ExportPropagator,
    },
    root::reset::ResetBang,
    root::export::signal::ExportBang,
};

use self::prelude::{ActuatorPropagator, ReleasePropagator, RootBang};

#[derive(Bundle)]
#[derive(Default)]
pub struct NodeBundle {
    pub bang: Bang,
    pub state: TState, // State terminal
    pub state_output: StateOutput,

    pub latch_propagator: LatchPropagator,
    pub release_propagator: ReleasePropagator,
    pub actuator_propagator: ActuatorPropagator,
    pub export_propagator: ExportPropagator,

    // Waymarks
    pub to_parent: ToParentNode,
    pub to_root: ToBehaviourRoot,
}

#[derive(Bundle)]
#[derive(Default)]
/// Additionally, add either one of these: ExportWhenCount, ExportForCount.
pub struct RootBundle {
    pub tree_bang: RootBang,
    pub reset_bang: ResetBang,
    pub export_bang: ExportBang,

    // Waymarks
    pub tree_exit: BehaviourTreeExit
}