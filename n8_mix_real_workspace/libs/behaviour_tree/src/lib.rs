pub mod state;
pub mod bang;
pub mod root;

use bevy::prelude::*;
use ref_paths::*;

#[derive(Component)]
pub struct ToParentNode(Entity);
waymark!(ToParentNode);

#[derive(Component)]
pub struct ToBehaviourRoot(Entity);
waymark!(ToBehaviourRoot);

#[derive(Component)]
pub struct BehaviourTreeExit(Entity);
waymark!(BehaviourTreeExit);