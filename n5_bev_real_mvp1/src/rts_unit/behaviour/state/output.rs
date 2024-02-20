
use bevy_rapier2d::prelude::Group as State;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::rts_unit::{
    *,
    behaviour::*,
};
use super::*;

#[derive(Component)]
/// Node's behaviour state output terminal
/// Concocts local insertion and removal, and outputs it to the parent as a single insertion
/// Key'd via the entity this is on
pub struct TBehaviourStateOutput {
    change_flag: ChangeFlag,
    held_state: State,
    registered_state: HashMap<ChangeKey, State>,
    insert: HashMap<ChangeKey, State>, // Insert into registered
    remove: Vec<ChangeKey>, // Remove from registered
}
t_behaviour_state_impls!(TBehaviourStateOutput);

impl TypeIdGet for TBehaviourStateOutput { } 
impl EntityReferenceFlag<2, BehaviourTreeNode> for TBehaviourStateOutput {
    const REFERENCE_PATH: [TypeId; 2] = [ToParentNode::TYPE_ID, BehaviourTreeNode::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Mutable;
}

fn state_output_updates(
    mut node_q: Query<(&mut TBehaviourStateOutput, &ToParentNode, Entity), Changed<TBehaviourStateOutput>>,
    mut parent_q: Query<&mut TBehaviourState>,
) {
    for (state_output, to_parent, node) in node_q.iter_mut() {
        state_output_update(&mut parent_q, state_output, to_parent.entity(), node);
    }
}

fn state_output_update(
    parent_q: &mut Query<&mut TBehaviourState>,
    mut state_output: Mut<TBehaviourStateOutput>,
    parent: Entity,
    node: Entity
) {
    if state_output.insertion() {
        state_output.insertion_to_registered();
    }

    if state_output.removal() {
        state_output.removal_to_registered();
    }

    match state_output.change_flag() {
        ChangeFlag::HasChanged => { 
            // continue
        },
        ChangeFlag::Lowered => { 
            return;
        }, 
    }

    let parent_state = parent_q.get_mut(parent);
    let Ok(mut parent_state) = parent_state else {
        TBehaviourStateOutput::print_err_descript(1, "Failed at getting the parent's behaviour state terminal.");
        return;
    };

    parent_state.insert_state(ChangeKey::Entity(node), state_output.state());
}