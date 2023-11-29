/// Handles the logic for completing a unit's current order

use bevy::prelude::*;
use super::*;
use super::commander::*;

use std::collections::VecDeque;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit::order_processing");
    }
}

#[derive(Clone, Copy)]
pub struct Order {
    pub recieving_unit: Entity,
    pub order_type: OrderType,
    pub waypoint: Vec2,
    pub target_unit: Entity,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum OrderType {
    ClearOrderList,
    PureMovement,
    AttackMove,
    AttackTarget,
}