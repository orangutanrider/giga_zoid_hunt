use bevy::prelude::*;

use crate::*;

#[derive(Component, Default)]
pub struct LocalTransformMovement;

pub fn tranform_movement_sys(
    mut q: Query<(&mut Transform, &TMoveVector, Option<&MoveSpeed>), With<LocalTransformMovement>>,
    time: Res<Time>,
) {
    for (mut transform, terminal, speed) in q.iter_mut() {
        const MOVEMENT_TYPE_POWER: f32 = 100.0;

        let speed = match speed {
            Some(v) => v.read(),
            None => 1.0,
        };

        let move_vector = terminal.0 * MOVEMENT_TYPE_POWER * speed; 
        let time_adjusted = move_vector * time.delta_seconds();
        let new_position = transform.translation + time_adjusted.extend(0.0);
        
        transform.translation = new_position;
    }
}

pub fn tranform_movement_aggregator_sys(
    mut q: Query<(&mut Transform, &TMoveAggregator, Option<&MoveSpeed>), With<LocalTransformMovement>>,
    time: Res<Time>,
) {
    for (mut transform, terminal, speed) in q.iter_mut() {
        const MOVEMENT_TYPE_POWER: f32 = 100.0;

        let speed = match speed {
            Some(v) => v.read(),
            None => 1.0,
        };

        let move_vector = terminal.read() * MOVEMENT_TYPE_POWER * speed; 
        let time_adjusted = move_vector * time.delta_seconds();
        let new_position = transform.translation + time_adjusted.extend(0.0);
        
        transform.translation = new_position;
    }
}

pub fn tranform_movement_decider_sys(
    mut q: Query<(&mut Transform, &TMoveDecider, Option<&MoveSpeed>), With<LocalTransformMovement>>,
    time: Res<Time>,
) {
    for (mut transform, terminal, speed) in q.iter_mut() {
        const MOVEMENT_TYPE_POWER: f32 = 100.0;

        let speed = match speed {
            Some(v) => v.read(),
            None => 1.0,
        };

        let move_vector = terminal.read() * MOVEMENT_TYPE_POWER * speed; 
        let time_adjusted = move_vector * time.delta_seconds();
        let new_position = transform.translation + time_adjusted.extend(0.0);
        
        transform.translation = new_position;
    }
}