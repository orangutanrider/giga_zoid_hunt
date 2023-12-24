/// Commanders handle the creation of orders for units

mod player;

use bevy::prelude::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit::commander");
    }
}

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
pub struct PureMovement {
    pub waypoint: Vec2,
}

#[derive(Clone, Copy)]
pub struct AttackMove{
    pub waypoint: Vec2,
}

#[derive(Clone, Copy)]
pub struct AttackTarget {
    pub target_unit: Entity,
}