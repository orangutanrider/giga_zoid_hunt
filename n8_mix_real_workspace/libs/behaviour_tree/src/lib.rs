pub mod state;
pub mod bang;
pub mod root;

use bevy::prelude::*;
use ref_paths::*;

pub mod plugins;
pub mod internal_systems;
pub mod external_systems;
pub mod prelude;

#[derive(Component)]
pub struct ToParentNode(Entity);
waymark!(ToParentNode);

#[derive(Component)]
pub struct ToBehaviourRoot(Entity);
waymark!(ToBehaviourRoot);

#[derive(Component)]
pub struct BehaviourTreeExit(Entity);
waymark!(BehaviourTreeExit);