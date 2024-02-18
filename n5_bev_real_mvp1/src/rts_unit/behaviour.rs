mod attack;
mod navigation;
mod state;

//pub mod blocks;
pub mod parts;

use bevy::prelude::*;

use crate::rts_unit::*;
use crate::entity_ref_impls;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        /* 
        app.add_plugins((
            navigation::InitializePlugin,
            detection::InitializePlugin,
            order_processing::InitializePlugin,
        ));
        */
    }
}

// Node definitions
#[derive(Clone, Copy)]
#[derive(Component)]
pub struct BehaviourTreeNode(Entity);
entity_ref_impls!(BehaviourTreeNode, SelfEntity);

#[derive(Component)]
pub struct ToParentNode(Entity);
entity_ref_impls!(ToParentNode, ChildEntity);


// Root definitions
#[derive(Clone, Copy)]
#[derive(Component)]
pub struct RTSUnitBehaviour(Entity);
entity_ref_impls!(RTSUnitBehaviour, SelfEntity);

#[derive(Component)]
pub struct RootToBehaviour(Entity);
entity_ref_impls!(RootToBehaviour, ChildEntity);

#[derive(Component)]
pub struct ChildToBehaviour(Entity);
entity_ref_impls!(ChildToBehaviour, ParentEntity);