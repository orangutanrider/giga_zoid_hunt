pub mod state;
pub mod bang;
pub mod root;

use bevy::prelude::*;
use ref_paths::*;

//pub mod prelude;
//pub mod bundles;

#[derive(Component)]
pub struct ToParentNode(Entity);
waymark!(ToParentNode);

#[derive(Component)]
pub struct ToBehaviourRoot(Entity);
waymark!(ToBehaviourRoot);

#[derive(Component)]
pub struct BehaviourTreeExit(Entity);
waymark!(BehaviourTreeExit);