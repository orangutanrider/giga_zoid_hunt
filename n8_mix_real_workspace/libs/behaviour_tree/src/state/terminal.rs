use bevy::prelude::*;
use bevy::utils::HashMap;

use super::State;
use super::Key;

#[derive(Component)]
/// Behaviour State Terminal
/// Collects and stores key'd behaviour state.
pub struct TState {
    /// True if the held state has changed
    changed: bool, 
    /// Overall held state, collected from all registered state input
    held: State,
    /// Key'd individual state inputs
    registered: HashMap<Key, State>,
}
impl Default for TState {
    fn default() -> Self {
        return Self::new()
    }
}
impl TState { //! Constructor
    pub fn new() -> Self {
        return Self {
            changed: false,
            held: State::NONE,
            registered: HashMap::new(),
        }
    }
}

impl TState { //! Set
    /// Adds a new entry or updates an existing entry
    pub fn insert(&mut self, key: Key, state: State) {
        // Input
        let insert = self.registered.insert(key, state);
        // Check for change
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
impl TState { //! Get
    pub fn state(&self) -> State {
        return self.held
    }

    pub fn changed(&self) -> bool {
        return self.changed
    }
}

impl TState { //! Internal
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