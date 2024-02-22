pub mod activation;
#[macro_use]
pub mod output;

use bevy_rapier2d::prelude::Group as State;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::rts_unit::*;
use super::*;

#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
pub enum ChangeKey {
    Entity(Entity),
    Component(TypeId),
}

#[derive(Clone, Copy)]
pub enum ChangeFlag {
    HasChanged,
    Lowered, // Has not changed
}

#[macro_export]
macro_rules! t_behaviour_state_impls { ($t: ty) => {
    impl Default for $t {
        fn default() -> Self {
            Self { 
                change_flag: ChangeFlag::Lowered,
                held_state: State::NONE,
                registered_state: HashMap::new(), 
                insert: HashMap::new(),
                remove: Vec::new(),
            }
        }
    }
    impl $t {
        fn new() -> Self {
            return Self::default()
        }
    }
    
    /// Set Methods
    impl $t {
        /// Add new, or update existing, state
        /// Key should identify the source that is inserting this state, so that insertions are not duplicated
        pub fn insert_state(&mut self, key: ChangeKey, state: State) {
            self.insert.insert(key, state);
        }
    
        /// Explicitlly remove a registered state entry, via its key
        pub fn remove_state(&mut self, key: ChangeKey) {
            self.remove.push(key);
        }
    }
    
    /// Get Methods
    impl $t {
        pub fn state(&self) -> State {
            return self.held_state
        }
    
        pub fn change_flag(&self) -> ChangeFlag {
            return self.change_flag
        }
    }
    
    /// Internal
    impl $t {
        fn insertion(&self) -> bool {
            return self.insert.len() != 0;
        }
    
        fn removal(&self) -> bool {
            return self.remove.len() != 0;
        }
    
        fn insertion_to_registered(&mut self) {
            let registered = &mut self.registered_state;
            let insert = &mut self.insert;
            for (k, v) in insert.iter() {
                registered.insert(*k, *v);
            }
            insert.clear();
    
            let state = State::NONE;
            for (k, v) in registered.iter() {
                state.union(*v);
            }
    
            if state == self.held_state {
                self.change_flag = ChangeFlag::Lowered; // If state has not changed
            }
    
            self.held_state = state;
            self.change_flag = ChangeFlag::HasChanged;
        }
    
        fn removal_to_registered(&mut self) {
            let registered = &mut self.registered_state;
            let remove = &mut self.remove;
            for k in remove.iter() {
                registered.remove(k);
            }
            remove.clear();
    
            let state = State::NONE;
            for (k, v) in registered.iter() {
                state.union(*v);
            }
    
            if state == self.held_state {
                self.change_flag = ChangeFlag::Lowered; // If state has not changed
            }
    
            self.held_state = state;
            self.change_flag = ChangeFlag::HasChanged;
        }
    }
};}
pub(crate) use t_behaviour_state_impls;

#[derive(Component)]
/// Node's behaviour state terminal
pub struct TBehaviourState {
    change_flag: ChangeFlag,
    held_state: State,
    registered_state: HashMap<ChangeKey, State>,
    insert: HashMap<ChangeKey, State>, // Insert into registered
    remove: Vec<ChangeKey>, // Remove from registered
}
t_behaviour_state_impls!(TBehaviourState);

fn state_updates(
    mut node_q: Query<&mut TBehaviourState, Changed<TBehaviourState>>,
) {
    for state in node_q.iter_mut() {
        state_update(state);
    }
}

fn state_update(
    mut state: Mut<TBehaviourState>
) {
    if state.insertion() {
        state.insertion_to_registered();
    }

    if state.removal() {
        state.removal_to_registered();
    }
}