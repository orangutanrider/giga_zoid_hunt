/// The player's unit commander

use bevy::prelude::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit::commander::player");
    }
}

#[derive(Resource)]
pub struct PlayerUnitCommander {
    pushed_commands: Vec<Command>,
}
impl PlayerUnitCommander {
    pub fn command_clear_orders(
        &mut self,
        unit: Unit,
    ) {
        self.pushed_commands.push(Command { 
            recieving_unit: unit.entity, 
            command_type: CommandType::ClearOrderList, 
            waypoint: Vec2::ZERO,
            target_unit: Entity::PLACEHOLDER,
        });
    }

    pub fn command_pure_movement(
        &mut self,
        unit: Unit,
        waypoint: Vec2,
    ) {
        self.pushed_commands.push(Command { 
            recieving_unit: unit.entity, 
            command_type: CommandType::PureMovement, 
            waypoint,
            target_unit: Entity::PLACEHOLDER,
        });
    }
    
    pub fn command_attack_move(
        &mut self,
        unit: Unit,
        waypoint: Vec2,
    ) {
        self.pushed_commands.push(Command { 
            recieving_unit: unit.entity, 
            command_type: CommandType::AttackMove, 
            waypoint,
            target_unit: Entity::PLACEHOLDER,
        });
    }
    
    pub fn command_attack_target(
        &mut self,
        unit: Unit,
        target: Unit,
    ){
        self.pushed_commands.push(Command {
            recieving_unit: unit.entity, 
            command_type: CommandType::AttackTarget, 
            waypoint: Vec2::ZERO,
            target_unit: target.entity,
        });
    }
}

fn process_command_pushes(
    mut context: ResMut<PlayerUnitCommander>,
    mut q: Query<&mut Commandable>,
) {
    if context.pushed_commands.len() == 0 {
        return;
    }

    // process commands
    for command in context.pushed_commands.iter() {
        // get commandable
        let commandable = q.get_mut(command.recieving_unit);
        let mut commandable = commandable.unwrap();

        // spawn order
        let mut new_order: Entity = Entity::PLACEHOLDER;
        match command.command_type {
            CommandType::ClearOrderList => {
                clear_orders(&mut commandable);
                continue;
            }
            CommandType::PureMovement => {
                new_order = spawn_pure_movement();
            }
            CommandType::AttackMove => {
                new_order = spawn_attack_move();
            }
            CommandType::AttackTarget => {
                new_order = spawn_attack_target();
            }
        }

        // append commandable data with new order
    }

    // Clear commands, they have now been processed
    context.pushed_commands.clear();
}

// Internal
fn clear_orders(commandable: &mut Commandable) {
    todo!()
}

fn spawn_pure_movement(

) -> Entity {
    todo!()
}

fn spawn_attack_move(

) -> Entity {
    todo!()
}

fn spawn_attack_target(

) -> Entity {
    todo!()
}