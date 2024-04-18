use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::*;

#[derive(Component, Default)]
pub struct LocalVelocityMovement;
const MOVEMENT_TYPE_POWER: f32 = 100.0;

pub fn velocity_movement_sys(
    mut q: Query<(&mut Velocity, &TMoveVector, Option<&MoveSpeed>), With<LocalVelocityMovement>>,
    time: Res<Time>,
) {
    for (mut velocity, terminal, speed) in q.iter_mut() {

        let speed = match speed {
            Some(v) => v.read(),
            None => 1.0,
        };

        let move_vector = terminal.0 * MOVEMENT_TYPE_POWER * speed; 
        //let time_adjusted = move_vector * time.delta_seconds();

        velocity.linvel = move_vector;
    }
}

pub fn velocity_movement_aggregator_sys(
    mut q: Query<(&mut Velocity, &TMoveAggregator, Option<&MoveSpeed>), With<LocalVelocityMovement>>,
    time: Res<Time>,
) {
    for (mut velocity, terminal, speed) in q.iter_mut() {

        let speed = match speed {
            Some(v) => v.read(),
            None => 1.0,
        };

        let move_vector = terminal.read() * MOVEMENT_TYPE_POWER * speed; 
        //let time_adjusted = move_vector * time.delta_seconds();

        velocity.linvel = move_vector;
    }
}

pub fn velocity_movement_decider_sys(
    mut q: Query<(&mut Velocity, &TMoveDecider, Option<&MoveSpeed>), With<LocalVelocityMovement>>,
    time: Res<Time>,
) {
    for (mut velocity, terminal, speed) in q.iter_mut() {

        let speed = match speed {
            Some(v) => v.read(),
            None => 1.0,
        };

        let move_vector = terminal.read() * MOVEMENT_TYPE_POWER * speed; 
        //let time_adjusted = move_vector * time.delta_seconds();

        velocity.linvel = move_vector;
    }
}