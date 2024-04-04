pub mod transform;

use bevy::prelude::*;

#[derive(Component)]
/// Data terminal.
pub struct TMoveVector(pub Vec2);