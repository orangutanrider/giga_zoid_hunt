use crate::unit_order_terminal;
use super::*;

#[derive(Clone, Copy)]
pub struct PureMovementOrder {
    pub waypoint: Vec2,
}
impl Default for PureMovementOrder {
    fn default() -> Self {
        return Self { 
            waypoint: Vec2::ZERO,
        }
    }
}
impl PureMovementOrder {
    pub fn new(waypoint: Vec2) -> Self {
        return Self { waypoint }
    }
}

#[derive(Component)]
/// Pure movement order terminal
pub struct TPureMovementOrders(Vec<PureMovementOrder>);
unit_order_terminal!(TPureMovementOrders, PureMovementOrder);