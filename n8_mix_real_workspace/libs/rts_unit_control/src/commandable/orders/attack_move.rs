pub mod processing;

use crate::{commander::WaypointOrder, unit_order_terminal};
use super::*;

#[derive(Clone, Copy, Debug)]
pub struct AttackMoveOrder {
    pub waypoint: Vec2,
}
impl Default for AttackMoveOrder {
    fn default() -> Self {
        return Self { 
            waypoint: Vec2::ZERO,
        }
    }
}
impl WaypointOrder for AttackMoveOrder {
    fn waypoint(&self) -> Vec2 {
        return self.waypoint
    }

    fn from_waypoint(waypoint: Vec2) -> Self {
        return Self::new(waypoint)
    }
}
impl AttackMoveOrder {
    pub fn new(waypoint: Vec2) -> Self {
        return Self { waypoint }
    }
}

#[derive(Component)]
/// Attack movement order terminal
pub struct TAttackMoveOrders(VecDeque<AttackMoveOrder>);
unit_order_terminal!(TAttackMoveOrders, AttackMoveOrder);