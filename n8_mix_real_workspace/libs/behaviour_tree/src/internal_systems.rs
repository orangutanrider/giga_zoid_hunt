//! Systems that the library's plugins already integrate.
//! They are public, in case you want to define your own plugins.

pub mod internal;

pub use crate::state::output::state_output_sys;

pub use crate::bang::{
    bang_propogation_sys,
    bang_update_to_root_sys,
    latch::{
        latch_propagation_sys,
        basic_latch_sys,
    },
    reference::export_propogation_sys
};

pub use crate::root::{
    reset_behaviour_sys, // This one is both internal and external
    export::signal::{
        export_when_count_sys,
        export_for_count_sys
    }
};