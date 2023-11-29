/// Handles callbacks for giving orders to units

use bevy::prelude::*;
use super::*;
use super::order_processing::*;

use std::collections::VecDeque;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit::commander");
        app
        .init_resource::<CommanderContext>()
        .add_systems(Update, process_command_pushes);
    }
}

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Commandable {
    pub orders: VecDeque<Order>,
}
impl Default for Commandable {
    fn default() -> Self {
        Self { 
            orders: VecDeque::new(),
        }
    }
}

#[derive(Resource, Default)]
pub struct CommanderContext {
    pushed_commands: Vec<Order>,
}
impl CommanderContext {
    pub fn command_clear_orders(
        &mut self,
        unit: Unit,
    ) {
        self.pushed_commands.push(Order { 
            recieving_unit: unit.entity, 
            order_type: OrderType::ClearOrderList, 
            waypoint: Vec2::ZERO,
            target_unit: Entity::PLACEHOLDER,
        });
    }

    pub fn command_pure_movement(
        &mut self,
        unit: Unit,
        waypoint: Vec2,
    ) {
        self.pushed_commands.push(Order { 
            recieving_unit: unit.entity, 
            order_type: OrderType::PureMovement, 
            waypoint,
            target_unit: Entity::PLACEHOLDER,
        });
    }
    
    pub fn command_attack_move(
        &mut self,
        unit: Unit,
        waypoint: Vec2,
    ) {
        self.pushed_commands.push(Order { 
            recieving_unit: unit.entity, 
            order_type: OrderType::AttackMove, 
            waypoint,
            target_unit: Entity::PLACEHOLDER,
        });
    }
    
    pub fn command_attack_target(
        &mut self,
        unit: Unit,
        target: Unit,
    ){
        self.pushed_commands.push(Order { 
            recieving_unit: unit.entity, 
            order_type: OrderType::AttackTarget, 
            waypoint: Vec2::ZERO,
            target_unit: target.entity,
        });
    }
}

fn process_command_pushes(
    mut context: ResMut<CommanderContext>,
    mut q: Query<&mut Commandable>,
) {
    if context.pushed_commands.len() == 0 {
        return;
    }

    for command in context.pushed_commands.iter() {
        // Get commandable
        let commandable = q.get_mut(command.recieving_unit);
        let mut commandable = commandable.unwrap();

        // Give unit order
        commandable.orders.push_back(*command);
    }

    // Clear commands, they have now been processed
    context.pushed_commands.clear();
}