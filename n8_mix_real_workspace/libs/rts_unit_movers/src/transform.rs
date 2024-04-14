use bevy::prelude::*;

use crate::TMoveVector;

#[derive(Component, Default)]
pub struct LocalTransformMovement;

pub fn tranform_movement_sys(
    mut q: Query<(&mut Transform, &TMoveVector), With<LocalTransformMovement>>,
    time: Res<Time>,
) {
    for (mut transform, terminal) in q.iter_mut() {
        const MOVEMENT_TYPE_POWER: f32 = 100.0;

        let move_vector = terminal.0 * MOVEMENT_TYPE_POWER; 
        let time_adjusted = move_vector * time.delta_seconds();
        let new_position = transform.translation + time_adjusted.extend(0.0);
        
        transform.translation = new_position;
    }
}