pub mod transform;
pub use transform::*;

use bevy::prelude::*;
use ref_paths::*;

pub struct MoversPlugin;

impl Plugin for MoversPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tranform_movement_sys);
    }
}

#[derive(Component)]
pub struct ToMover(Entity);
waymark!(ToMover);

#[derive(Component)]
/// Data terminal.
pub struct TMoveVector(pub Vec2);