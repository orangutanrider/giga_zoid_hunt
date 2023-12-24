/// unit's AIs follow orders that're given to them from external sources

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct OrderCore {
    pub order_type: OrderType,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum OrderType {
    PureMovement,
    AttackMove,
    AttackTarget,
}

// ATTACK TARGET
#[derive(Clone, Copy)]
pub struct AttackTarget {
    pub invalidated: bool,
    pub target_unit: Entity,
}

// ATTACK MOVE
#[derive(Clone, Copy)]
pub struct AttackMove {
    pub waypoint: Vec2,
}

// PURE MOVEMENT
#[derive(Clone, Copy)]
pub struct PureMovement {
    pub waypoint: Vec2,
}