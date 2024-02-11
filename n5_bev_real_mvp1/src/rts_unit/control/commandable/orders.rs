use bevy::prelude::*;

use crate::rts_unit::soul::RTSUnitSoul;

#[derive(Clone, Copy)]
pub enum OrderType {
    PureMovement(PureMovementOrder),
    AttackMove(AttackMoveOrder),
    AttackTarget(AttackTargetOrder),
    Empty,
}

#[derive(Clone, Copy)]
pub struct RTSUnitOrder {
    pub order_type: OrderType,
}
impl Default for RTSUnitOrder {
    fn default() -> Self {
        Self { 
            order_type: OrderType::Empty, 
        }
    }
}
impl RTSUnitOrder {
    pub fn new(order_type: OrderType) -> Self {
        return Self{
            order_type
        }
    }
}

// ATTACK TARGET
#[derive(Clone, Copy)]
pub struct AttackTargetOrder {
    pub target: Option<RTSUnitSoul>,
}
impl Default for AttackTargetOrder {
    fn default() -> Self {
        return Self {
            target: None, 
        }
    }
}
impl AttackTargetOrder {
    pub fn new(target: RTSUnitSoul) -> Self {
        return Self {
            target: Some(target), 
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
        return Self { 
            waypoint: Vec2::ZERO,
        }
    }
}
impl AttackMoveOrder {
    pub fn new(waypoint: Vec2) -> Self {
        return Self { waypoint }
    }

    pub fn is_within_distance_of(&self, distance: f32, position: Vec2) -> bool {
        if self.waypoint.distance(position) <= distance {
            return true;
        }
        return false;
    }
}

// PURE MOVEMENT
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

    pub fn is_within_distance_of(&self, distance: f32, position: Vec2) -> bool {
        if self.waypoint.distance(position) <= distance {
            return true;
        }
        return false;
    }
}