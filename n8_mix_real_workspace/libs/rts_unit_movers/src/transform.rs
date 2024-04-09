use bevy::prelude::*;

use crate::TMoveVector;

#[derive(Component)]
pub struct LocalTransformMovement;

pub fn tranform_movement_sys(
    mut q: Query<(&mut Transform, &TMoveVector), With<LocalTransformMovement>>
) {
    for (mut transform, terminal) in q.iter_mut() {
        let new_position = transform.translation + terminal.0.extend(0.0);
        transform.translation = new_position;
    }
}