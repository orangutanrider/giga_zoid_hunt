use bevy::prelude::*;

use crate::unit::movement::*;
use super::selection::{*, self};

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing gameplay_controller::unit_orders");
        app
            .add_systems( Update, (
                process_new_orders,
            ))
        ;
    }
}

#[derive(Component)]
pub struct NewOrderManager{
    order_pushes: Vec<Order>,
}

#[derive(Clone, Copy)]
pub struct Order{
    pub move_to_point: Vec2,
}

// Callback processing
fn process_new_orders(
    input: Res<Input<KeyCode>>,
    mut manager_q: Query<&mut NewOrderManager>,
    selection_q: Query<&mut UnitSelection>,
    mut movement_q: Query<&mut UnitMovement>,
){
    let mut manager = manager_q.single_mut();

    if manager.order_pushes.len() == 0 { // If no new orders, exit
        return;
    }
    
    let selection = selection_q.single();

    for order in manager.order_pushes.iter() { // for each new order
        if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]){ // if shift isn't pressed, clear the selection's order lists
            clear_orders(&mut movement_q, selection);
        }
    
        give_order(&mut movement_q, selection, *order);
    }

    manager.order_pushes.clear(); 
}

// Internal
fn clear_orders(
    movement_q: &mut Query<&mut UnitMovement>,
    selection: &UnitSelection,
) {
    for unit_entity in selection.selection.iter(){
        let movement = movement_q.get_mut(*unit_entity);
        let movement = &mut movement.unwrap();
        
        movement.waypoints.clear();
    }
}

fn give_order(
    movement_q: &mut Query<&mut UnitMovement>,
    selection: &UnitSelection,
    order: Order,
) {
    for unit_entity in selection.selection.iter(){
        let movement = movement_q.get_mut(*unit_entity);
        let movement = &mut movement.unwrap();

        movement.waypoints.push(
            // this sucks, make waypoint and order equal the same thing in the refactor
            Waypoint { point:order.move_to_point }
        );
    }
}

// Callbacks
pub fn give_movement_order(
    manager: &mut NewOrderManager,
    order: Order,
) {
    manager.order_pushes.push(order);
}