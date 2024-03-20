use bevy::prelude::*;
use bevy::utils::HashMap;

use ref_caravan::ref_caravan;
use ref_paths::*;

use crate::ToParentNode;
use super::{
    State,
    *
};
use terminal::TState;

#[derive(Component)]
/// Collects local state inputs and outputs it as a single HashMap entry
/// To the parent node's state terminal, key'd via entity
pub struct StateOutput {
    /// True if the held state has changed, on change output to parent
    changed: bool, 
    /// Overall held state, collected from all registered state input
    held: State,
    /// Key'd individual state inputs
    registered: HashMap<Key, State>,
}
impl Default for StateOutput {
    fn default() -> Self {
        return Self::new()
    }
}
impl StateOutput { //! Constructor
    pub fn new() -> Self {
        return Self {
            changed: false,
            held: State::NONE,
            registered: HashMap::new(),
        }
    }
}

impl StateOutput { //! Set
    /// Adds a new entry or updates an existing entry
    pub fn insert(&mut self, key: Key, state: State) {
        let insert = self.registered.insert(key, state);
        match insert {
            Some(old) => {
                if old == state { return; } // If value is different, re_calculate
                self.re_calculate();
            },
            None => {
                self.re_calculate();
            },
        }
    }

    /// Explicitlly remove an entry
    pub fn remove(&mut self, key: &Key) {
        self.registered.remove(key);
    }
}
impl StateOutput { //! Get
    pub fn state(&self) -> State {
        return self.held
    }

    pub fn changed(&self) -> bool {
        return self.changed
    }
}

impl StateOutput { //! Internal
    fn re_calculate(&mut self) {
        // Collect new held
        let mut new_held = State::NONE;
        for v in self.registered.iter() {
            new_held = new_held.union(*v.1);
        }
        
        // Check if it is different
        if new_held == self.held {
            self.changed = false;
            return;
        }

        // Change held if it was different
        self.held = new_held;
        self.changed = true;
    }
}

/// Will take the inputs of StateOutput components and input them into the parent's state terminal.
/// Key'd via entity.
pub fn state_output_sys (
    mut node_q: Query<(&mut StateOutput, &ToParentNode, Entity), Changed<StateOutput>>,
    mut parent_q: Query<&mut TState>,
) {
    for (mut output, to_parent, id) in node_q.iter_mut() {
        // If changed, continue
        if !output.changed {
            output.changed = true;
            continue;
        }
        output.changed = true;
        
        state_output(output, to_parent, id, &mut parent_q);
    }
}

pub fn state_output(
    output: Mut<StateOutput>, to_parent: &ToParentNode, id: Entity,
    parent_q: &mut Query<&mut TState>,
) {
    ref_caravan!(
        to_parent::parent_q(mut terminal);
    );

    terminal.insert(Key::ExternalEntity(id), output.held)
}