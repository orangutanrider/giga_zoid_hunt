use bevy::prelude::*;
use ref_paths::*;

#[derive(Component)]
pub struct ToRoot(Entity);
waymark!(ToRoot);