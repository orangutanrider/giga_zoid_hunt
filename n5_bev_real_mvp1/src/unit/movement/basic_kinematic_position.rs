use bevy:: prelude::*;
use super::{BasicMover, KinematicPositionBasicMoverAugment};

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_update);
    }
}

fn move_update(
    mover_q: Query<&BasicMover>,
    mut aug_q: Query<&mut KinematicPositionBasicMoverAugment, With<BasicMover>>,
    mut transform_q: Query<&mut Transform>,
) {
    for aug in aug_q.iter_mut() {
        movement(&mover_q, &mut transform_q, aug);
    }
}

/// https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
fn movement(
    mover_q: & Query<&BasicMover>,
    transform_q: &mut Query<&mut Transform>,
    aug: Mut<'_, KinematicPositionBasicMoverAugment>,
) {
    let entity = aug.entity;
    let transform = transform_q.get_mut(entity);
    let mut transform = transform.unwrap();
    let mover = mover_q.get(entity);
    let mover = mover.unwrap();
    let new_position = transform.translation + (mover.read_move_vec().extend(0.0) * KinematicPositionBasicMoverAugment::AUG_GLOBAL_POWER);

    transform.translation = new_position;
}