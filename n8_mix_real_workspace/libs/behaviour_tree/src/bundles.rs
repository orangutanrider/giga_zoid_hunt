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
pub struct NodeBundle {
    bang: Bang,
    state: TState, // State terminal
    state_output: StateOutput,

    latch_propagator: LatchPropagator,
    release_propagator: ReleasePropagator,
    actuator_propagator: ActuatorPropagator,
    export_propagator: ExportPropagator,

    // Waymarks
    to_parent: ToParentNode,
    to_root: ToBehaviourRoot,
}

#[derive(Bundle)]
/// Additionally, add either one of these: ExportWhenCount, ExportForCount.
pub struct RootBundle {
    tree_bang: RootBang,
    reset_bang: ResetBang,
    export_bang: ExportBang,

    // Waymarks
    tree_exit: BehaviourTreeExit
}