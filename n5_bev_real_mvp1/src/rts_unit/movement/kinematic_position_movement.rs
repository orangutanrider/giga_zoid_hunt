use bevy:: prelude::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_update);
    }
}

#[derive(Component)]
pub struct KinematicPositionMover{
    mover_internal: MoverInternal,
}
impl Mover for KinematicPositionMover {
    fn read_move_vec(&self) -> Vec2 {
        return self.mover_internal.move_vec;
    }

    fn input_move_vec(&mut self, move_vec: Vec2) {
        self.mover_internal.move_vec = move_vec;
    }
}
impl KinematicPositionMover {
    const GLOBAL_POWER: f32 = 1.0;
    pub fn new (mover_power: f32) -> Self {
        return Self { 
            mover_internal: MoverInternal::new(mover_power),
        }
    }
}

fn move_update(
    mut q: Query<(&mut Transform, & KinematicPositionMover)>,
) {
    for (mut transform, mover) in q.iter_mut() {
        movement(transform, mover);
    }
}

/// https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
/// Kinematic bodies are moved via transform 
fn movement(
    mut transform: Mut<'_, Transform>,
    mover: &KinematicPositionMover,
) {
    let new_position = transform.translation + (mover.read_move_vec().extend(0.0) * KinematicPositionMover::GLOBAL_POWER);
    transform.translation = new_position;
}