use bevy_rapier2d::prelude::Group as State;
use bevy::prelude::*;

use crate::rts_unit::*;
use super::*;

macro_rules! active_on_parent_state_impls { ($t:ty) => {
    impl $t {
        pub fn new(
            state: State,
            method: StateComparisonMethod,
        ) -> Self {
            return Self { 
                state, 
                method, 
            }
        }
    }
    
    impl TypeIdGet for $t { } 
    impl EntityReferenceFlag<2, BehaviourTreeNode> for $t {
        const REFERENCE_PATH: [TypeId; 2] = [ToParentNode::TYPE_ID, BehaviourTreeNode::TYPE_ID];
        const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Immutable;
    }
    
    impl $t {
        pub fn check(&self, comparison: State) -> bool {
            let state = self.state; 
            match self.method {
                StateComparisonMethod::NonApplicable => { },
                StateComparisonMethod::Intersects => {
                    return state.intersects(comparison)
                },
                StateComparisonMethod::Contains => {
                    return state.contains(comparison)
                },
                StateComparisonMethod::Equals => {
                    return state == comparison
                },
            }
    
            return false
        }
    }
};}

#[derive(Clone, Copy)]
pub enum StateComparisonMethod {
    Intersects,
    Contains,
    Equals,
    NonApplicable, // ignored
}

#[derive(Component)]
pub struct NotActiveOnParentState{
    state: State, // If detected, will not be active, overwrites active
    method: StateComparisonMethod,
}
active_on_parent_state_impls!(NotActiveOnParentState);

#[derive(Component)]
pub struct ActiveOnParentState{
    state: State,
    method: StateComparisonMethod,
}
active_on_parent_state_impls!(ActiveOnParentState);

fn update_bangs_on_change(
    parent_q: Query<(&Children, &TBehaviourState), Changed<TBehaviourState>>,
    child_q: Query<(Option<&mut ActiveOnParentState>, Option<&mut NotActiveOnParentState>), Or<(With<ActiveOnParentState>, With<NotActiveOnParentState>)>>,
) {
    for (children, node_state) in parent_q.iter() {
        match node_state.change_flag() {
            ChangeFlag::HasChanged => { },
            ChangeFlag::Lowered => { continue; }, // If no change, skip
        }

        for child in children.iter() {
            update_bang(node_state, *child, child_q);
        }
    }
}

fn update_bang(
    parent: &TBehaviourState,
    child: Entity,
    child_q: Query<(Option<&mut ActiveOnParentState>, Option<&mut NotActiveOnParentState>), Or<(With<ActiveOnParentState>, With<NotActiveOnParentState>)>>
) {
    let results = child_q.get(child);
    let Ok((state_active, state_not_active)) = results else {
        return;
    };

    let parent_state = parent.state();

    let mut active_not = false;
    if let Some(state_not_active) = state_not_active {
        active_not = state_not_active.check(parent_state)
    }

    let mut active = false;
    if let Some(state_active) = state_active {
        active = state_active.check(parent_state)
    }

    if !active || active_not { // If child node not active
        todo!() // Update Bang
    }
    else { // If child node is active
        todo!() // Update Bang
    }
}

