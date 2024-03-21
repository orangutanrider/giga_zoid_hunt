//! If you want to re-define some of the systems.
//! You can access their internal functions here.

pub use crate::state::output::state_output;
pub use crate::bang::{
    bang_propogation,
    bang_update_to_root,
    latch::{
        latch_propagation,
        basic_latch_set_bang,
    },
    reference::export_propogation
};