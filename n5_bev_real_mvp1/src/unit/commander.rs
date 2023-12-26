/// Commanders handle the creation of orders for units, they give the unit's their data, that the then units store locally

mod player;

use bevy::prelude::*;
use super::*;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum CommandType {
    ClearOrders,
    PureMovement,
    AttackMove,
    AttackTarget,
}

#[derive(Clone, Copy)]
pub struct CommandCore {
    pub recieving_unit: Entity,
    pub command_type: CommandType,
}

#[derive(Clone, Copy)]
pub struct PureMovementCommand {
    pub waypoint: Vec2,
}

#[derive(Clone, Copy)]
pub struct AttackMoveCommand {
    pub waypoint: Vec2,
}

#[derive(Clone, Copy)]
pub struct AttackTargetCommand {
    pub target_unit: Entity,
}