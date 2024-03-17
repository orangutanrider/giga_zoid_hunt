pub mod state;

use bevy::prelude::*;
use ref_paths::*;

#[derive(Component)]
pub struct ToParentNode(Entity);
waymark!(ToParentNode);

#[derive(Component)]
pub struct ToBehaviourRoot(Entity);
waymark!(ToBehaviourRoot);