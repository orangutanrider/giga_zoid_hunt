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
    root::ResetBang,
    root::export::signal::ExportBang,
};

#[derive(Bundle)]
pub struct NodeBundle {
    bang: Bang,
    state: TState, // State terminal
    state_output: StateOutput,
    latch_propagator: LatchPropagator,
    export_propagator: ExportPropagator,

    // Waymarks
    to_parent: ToParentNode,
    to_root: ToBehaviourRoot,
}

#[derive(Bundle)]
pub struct RootBundle {
    reset_bang: ResetBang,
    export_bang: ExportBang,

    // Waymarks
    tree_exit: BehaviourTreeExit
}