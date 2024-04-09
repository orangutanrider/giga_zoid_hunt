pub mod transform;
pub use transform::*;

use bevy::prelude::*;
use ref_paths::*;

#[derive(Component)]
pub struct ToMover(Entity);
waymark!(ToMover);

#[derive(Component)]
/// Data terminal.
pub struct TMoveVector(pub Vec2);