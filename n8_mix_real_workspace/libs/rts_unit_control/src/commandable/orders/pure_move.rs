pub mod processing;

use crate::{commander::WaypointOrder, unit_order_terminal};
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
impl WaypointOrder for PureMoveOrder {
    fn waypoint(&self) -> Vec2 {
        return self.waypoint
    }

    fn from_waypoint(waypoint: Vec2) -> Self {
        return Self::new(waypoint)
    }
}
impl PureMoveOrder {
    pub fn new(waypoint: Vec2) -> Self {
        return Self { waypoint }
    }
}

#[derive(Component)]
/// Pure movement order terminal
pub struct TPureMoveOrders(VecDeque<PureMoveOrder>);
unit_order_terminal!(TPureMoveOrders, PureMoveOrder);