pub mod activation;

use bevy::core_pipeline::core_3d::graph::node;
use bevy_rapier2d::prelude::Group as State;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::rts_unit::*;
use super::*;

macro_rules! behaviour_state_node_impls { ($t:ty) => {
    impl Default for $t {
        fn default() -> Self {
            Self { 
                held_state: State::NONE,
                registered_state_input: HashMap::new(), 
                new_state_input: HashMap::new(),
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
        pub fn add_or_update_state(&mut self, id: TypeId, state: State) {
            self.new_state_input.insert(id, state);
        }
    
        pub fn remove_registered_state(&mut self, id: TypeId) {
            self.registered_state_input.remove(&id);
        }
    }
    
    /// Get Methods
    impl $t {
        pub fn state(&self) -> State {
            return self.held_state
        }
    }
    
    /// Internal
    impl $t {
        fn new_to_registered(&mut self) -> HeldStateChange {
            let registered = &mut self.registered_state_input;
            let new = &mut self.new_state_input;
            for (k, v) in new.iter() {
                registered.insert(*k, *v);
            }
            new.clear();
    
            let state = State::NONE;
            for (k, v) in registered.iter() {
                state.union(*v);
            }
    
            if state == self.held_state {
                return HeldStateChange::HasNotChanged // If state has not changed
            }
    
            self.held_state = state;
            return HeldStateChange::HasChanged
        }
    }
};}

#[derive(Clone, Copy)]
enum HeldStateChange {
    HasChanged,
    HasNotChanged,
}

#[derive(Component)]
/// Node's behaviour state output terminal
/// Outputs to the parent node
pub struct TNodeBehaviourStateOutput {
    held_state: State,
    registered_state_input: HashMap<TypeId, State>,
    new_state_input: HashMap<TypeId, State>,
}
behaviour_state_node_impls!(TNodeBehaviourStateOutput);

impl TypeIdGet for TNodeBehaviourStateOutput { } 
impl EntityReferenceFlag<2, BehaviourTreeNode> for TNodeBehaviourStateOutput {
    const REFERENCE_PATH: [TypeId; 2] = [ToParentNode::TYPE_ID, BehaviourTreeNode::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Mutable;
}

#[derive(Component)]
/// Node's behaviour state terminal
pub struct TNodeBehaviourState {
    held_state: State,
    registered_state_input: HashMap<TypeId, State>,
    new_state_input: HashMap<TypeId, State>,
}
behaviour_state_node_impls!(TNodeBehaviourState);

fn behaviour_node_state_updates(
    mut node_q: Query<&mut TNodeBehaviourState, Changed<TNodeBehaviourState>>,
) {
    for node_state in node_q.iter_mut() {
        behaviour_node_state_update(node_state);
    }
}

fn behaviour_node_state_update(
    mut node_state: Mut<TNodeBehaviourState>
) {
    let new_states = node_state.new_state_input;
    if new_states.is_empty() {
        return;
    }

    let change = node_state.new_to_registered();

    match change {
        HeldStateChange::HasChanged => {
            
        },
        HeldStateChange::HasNotChanged => {
            
        },
    }
}

