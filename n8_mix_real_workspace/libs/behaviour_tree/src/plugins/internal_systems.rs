//! Systems that the library's plugins already integrate.
//! They are public, in case you want to define your own plugins.

pub use crate::state::output::state_output_sys;

pub use crate::bang::{
    bang_update_to_root_sys,
    deactivation_propagation_sys,
    latch::{
        basic_latch_sys,
        state_to_latch_propagation_sys,
        bang_to_latch_propagation_sys,
        end_latch_propagation_sys,
    },
    actuator::{
        bang_to_actuator_propagation_sys,
        state_to_actuator_propagation_sys,
        end_actuator_propagation_sys,
    },
    release::{
        release_propagation_sys,
        end_release_propagation_sys,
    },
    reference::export_propogation_sys,
};

pub use crate::root::{
    bang::propagate_root_bang_sys,
    export::signal::{
        export_for_count_sys,
        export_when_count_sys,
    }
};