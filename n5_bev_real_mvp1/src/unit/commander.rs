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

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Commandable {
    pub idle: bool,
    pub current_order: Entity,
    pub final_order: Entity,
}
impl Default for Commandable{
    fn default() -> Self {
        Self { 
            idle: true, 
            current_order: Entity::PLACEHOLDER, 
            final_order: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Command {
    pub recieving_unit: Entity,
    pub command_type: CommandType,
    pub waypoint: Vec2,
    pub target_unit: Entity,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum CommandType {
    ClearOrderList,
    PureMovement,
    AttackMove,
    AttackTarget,
}