pub mod processing;

use crate::unit_order_terminal;
use super::*;

#[derive(Clone, Copy, Debug)]
pub struct PureMoveOrder {
    pub waypoint: Vec2,
}
impl Default for PureMoveOrder {
    fn default() -> Self {
        return Self { 
            waypoint: Vec2::ZERO,
        }
    }
}
impl PureMoveOrder {
    pub fn new(waypoint: Vec2) -> Self {
        return Self { waypoint }
    }
}

#[derive(Component)]
/// Pure movement order terminal
pub struct TPureMoveOrders(Vec<PureMoveOrder>);
unit_order_terminal!(TPureMoveOrders, PureMoveOrder);