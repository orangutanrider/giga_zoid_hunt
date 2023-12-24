/// unit's AIs follow orders that're given to them from external sources

use bevy::prelude::*;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum OrderType {
    PureMovement,
    AttackMove,
    AttackTarget,
    Empty
}

#[derive(Clone, Copy)]
pub struct OrderCore {
    pub order_type: OrderType,
}
impl OrderCore {
    pub const EMPTY: OrderCore = OrderCore {
        order_type: OrderType::Empty
    }; 
}
impl Default for OrderCore {
    fn default() -> Self {
        Self { 
            order_type: OrderType::Empty, 
        }
    }
}

// ATTACK TARGET
#[derive(Clone, Copy)]
pub struct AttackTarget {
    pub invalidated: bool,
    pub target_unit: Entity,
}
impl Default for AttackTarget {
    fn default() -> Self {
        Self {
            invalidated: false, 
            target_unit: Entity::PLACEHOLDER, 
        }
    }
}

// ATTACK MOVE
#[derive(Clone, Copy)]
pub struct AttackMove {
    pub waypoint: Vec2,
}
impl Default for AttackMove {
    fn default() -> Self {
        Self { 
            waypoint: Vec2::ZERO,
        }
    }
}

// PURE MOVEMENT
#[derive(Clone, Copy)]
pub struct PureMovement {
    pub waypoint: Vec2,
}
impl Default for PureMovement {
    fn default() -> Self {
        Self { 
            waypoint: Vec2::ZERO,
        }
    }
}