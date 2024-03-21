use bevy::{ecs::system::SystemParam, prelude::*};

use crate::state::terminal::TState;
use super::Bang;

#[derive(SystemParam)]
/// Standard query for bang fizzler systems
pub struct FizzlerQuery<'w, 's, Fizzler: Component>(
    pub Query<'w, 's, (&'static mut Bang, &'static TState), (With<Fizzler>, Changed<TState>)>
);

/// Prefab system for fizzlers that are flagged by a single component
pub fn bang_fizzler_sys<F, Fizzler: Component>(
    node_q: FizzlerQuery<Fizzler>,
    fizzler_logic: F
) where F: Fn(&TState) -> bool {
    let mut node_q = node_q.0;
    for (bang, state) in node_q.iter_mut() {
        fizzler_to_bang(bang, state, &fizzler_logic);
    }
}

/// Prefab function for bang fizzlers systems
pub fn fizzler_to_bang<F>(
    mut local_bang: Mut<Bang>,
    local_state: &TState,
    fizzler_logic: F
) where F: Fn(&TState) -> bool { 
    if !fizzler_logic(local_state) {
        return;
    }

    local_bang.fizzler_deactivate();
}