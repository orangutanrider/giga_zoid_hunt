use std::collections::VecDeque;

use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct UnitMovement{
    pub waypoints: VecDeque<Waypoint>,
}
impl UnitMovement{
    pub const DEFAULT_MOVEMENT_SPEED: f32 = 1.0;
}

#[derive(Clone, Copy)]
pub struct Waypoint{
    pub point: Vec2,
    // In future this will need to store data, like whether it is an attack move or not
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit::movement");
        app
           .add_systems(Update, handle_unit_movement)
        ;
    }
}

fn handle_unit_movement(
    mut q: Query<(&mut UnitMovement, &mut Transform), With<Unit>>
) {
    for (mut movement, mut transform) in q.iter_mut(){
        if movement.waypoints.len() == 0 {
            continue;
        }

        /* 
        let current_waypoint_pos = movement.waypoints[movement.waypoints.len()].point;
        if has_reached_waypoint(current_waypoint_pos, transform.translation.truncate()){

        } */

        move_towards_current_waypoint(&mut movement, &mut transform);
    }
}

// current waypoint is the last waypoint in the waypoint Vec<>
fn move_towards_current_waypoint(
    movement: &mut UnitMovement,
    transform: &mut Transform,
) {
    let waypoint_pos = movement.waypoints[0].point;
    let unit_pos = transform.translation.truncate();

    if waypoint_pos.distance(unit_pos) < UnitMovement::DEFAULT_MOVEMENT_SPEED {
        // Teleport exactly onto waypoint and remove current waypoint
        transform.translation = Vec3{x: waypoint_pos.x, y: waypoint_pos.y, z: transform.translation.z}; 
        movement.waypoints.pop_front();
        return;
    }

    // Move
    let movement_vector = (waypoint_pos - unit_pos).normalize() * UnitMovement::DEFAULT_MOVEMENT_SPEED;
    position_based_kinematic_movement(transform, movement_vector);
} 

/* 
fn has_reached_waypoint(
    waypoint: Vec2,
    current_position: Vec2,
) -> bool { 
    if waypoint.distance(current_position) < UnitMovement::DEFAULT_MOVEMENT_SPEED {
        return true;
    }
    return false;
} */

//https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies
fn position_based_kinematic_movement(
    transform: &mut Transform,
    movement_vector: Vec2,
){
    if movement_vector == Vec2::ZERO {
        return;
    }

    transform.translation = transform.translation + Vec3{x: movement_vector.x, y: movement_vector.y, z: 0.0};
}