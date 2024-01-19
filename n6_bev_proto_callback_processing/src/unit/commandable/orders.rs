/// Data structures for commandable units

use bevy::prelude::*;

#[derive(std::fmt::Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum OrderType {
    PureMovement,
    AttackMove,
    AttackTarget,
    Empty,
}

#[derive(Clone, Copy)]
pub struct OrderCore {
    pub index: usize,
    pub order_type: OrderType,
}
impl Default for OrderCore {
    fn default() -> Self {
        Self { 
            index: 0,
            order_type: OrderType::Empty, 
        }
    }
}

// ATTACK TARGET
#[derive(Clone, Copy)]
pub struct AttackTargetOrder {
    pub invalidated: bool,
    pub target_unit: Entity,
}
impl Default for AttackTargetOrder {
    fn default() -> Self {
        Self {
            invalidated: false, 
            target_unit: Entity::PLACEHOLDER, 
        }
    }
}

// ATTACK MOVE
#[derive(Clone, Copy)]
pub struct AttackMoveOrder {
    pub waypoint: Vec2,
}
impl Default for AttackMoveOrder {
    fn default() -> Self {
        Self { 
            waypoint: Vec2::ZERO,
        }
    }
}

// PURE MOVEMENT
#[derive(Clone, Copy)]
pub struct PureMovementOrder {
    pub waypoint: Vec2,
}
impl Default for PureMovementOrder {
    fn default() -> Self {
        Self { 
            waypoint: Vec2::ZERO,
        }
    }
}