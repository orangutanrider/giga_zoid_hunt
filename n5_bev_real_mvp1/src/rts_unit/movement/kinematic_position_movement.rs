use bevy:: prelude::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_update);
    }
}

#[derive(Component)]
pub struct KinematicPositionMovement;
impl Default for KinematicPositionMovement {
    fn default() -> Self {
        Self
    }
}
impl KinematicPositionMovement {
    const GLOBAL_POWER: f32 = 1.0;
    pub fn new () -> Self {
        return Self 
    }
}

fn move_update(
    mut q: Query<(&mut Transform, &TMover), With<KinematicPositionMovement>>,
) {
    for (transform, input) in q.iter_mut() {
        movement(transform, input.read());
    }
}

/// https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
/// Kinematic bodies are moved via transform 
fn movement(
    mut transform: Mut<'_, Transform>,
    move_input: Vec2,
) {
    let new_position = transform.translation + (move_input.extend(0.0) * KinematicPositionMovement::GLOBAL_POWER);
    transform.translation = new_position;
}