/// The player's unit commander, handles the callbacks for assigning orders to units

use bevy::prelude::*;
use crate::unit::orders::*;

use super::*;
use super::commandable::Commandable;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::commander::player");
        app.add_systems(Update, process_command_pushes);
    }
}

#[derive(Resource)]
pub struct PlayerUnitCommander {
    pushed_commands: Vec<CommandCore>,
    
    attack_moves: Vec<AttackMoveCommand>,
    pure_moves: Vec<PureMovementCommand>,
    attack_targets: Vec<AttackTargetCommand>,
}
impl PlayerUnitCommander {
    pub fn command_clear_orders(&mut self, unit: Entity) {
        self.pushed_commands.push(CommandCore { 
            recieving_unit: unit, 
            command_type: CommandType::ClearOrders, 
        });
    }

    pub fn command_pure_movement(&mut self, unit: Entity, pure_movement: PureMovementCommand) {
        self.pushed_commands.push(CommandCore { 
            recieving_unit: unit, 
            command_type: CommandType::PureMovement, 
        });
        self.pure_moves.push(pure_movement);
    }

    pub fn command_attack_move(&mut self, unit: Entity, attack_move: AttackMoveCommand) {
        self.pushed_commands.push(CommandCore { 
            recieving_unit: unit, 
            command_type: CommandType::AttackMove, 
        });
        self.attack_moves.push(attack_move);
    }

    pub fn command_attack_target(&mut self, unit: Entity, attack_target: AttackTargetCommand) {
        self.pushed_commands.push(CommandCore { 
            recieving_unit: unit, 
            command_type: CommandType::AttackTarget, 
        });
        self.attack_targets.push(attack_target);
    }
}

fn process_command_pushes(
    mut context: ResMut<PlayerUnitCommander>,
    mut q: Query<&mut Commandable>,
) {
    if context.pushed_commands.len() == 0 {
        return;
    }

    process_command_core(context, q);
}

fn process_command_core(
    mut context: ResMut<PlayerUnitCommander>,
    mut q: Query<&mut Commandable>,
) {
    // Recursively works through the list of commands, processing and clearing the Vecs

    let command_core = context.pushed_commands.pop();
    if command_core.is_none() {
        return;
    }
    let command_core = command_core.unwrap();
    
    match command_core.command_type {
        CommandType::ClearOrders=> process_clear_orders(command_core, &mut q),
        CommandType::PureMovement=> process_pure_movement(command_core, &mut context, &mut q),
        CommandType::AttackMove=> process_attack_move(command_core, &mut context, &mut q),
        CommandType::AttackTarget=> process_attack_target(command_core, &mut context, &mut q),
    }

    // Continue
    process_command_core(context, q);
}

fn process_clear_orders(
    command_core: CommandCore,
    q: &mut Query<&mut Commandable>,
){
    let commandable = q.get_mut(command_core.recieving_unit);
    let mut commandable = commandable.unwrap();
    let commandable = commandable.as_mut();

    commandable.clear_orders();
}

fn process_pure_movement(
    command_core: CommandCore,
    context: &mut ResMut<PlayerUnitCommander>,
    q: &mut Query<&mut Commandable>,
){
    let commandable = q.get_mut(command_core.recieving_unit);
    let mut commandable = commandable.unwrap();
    let commandable = commandable.as_mut();

    let pure_move = context.pure_moves.pop();
    let pure_move = pure_move.unwrap();
    let order = PureMovementOrder{
        waypoint: pure_move.waypoint,
    };

    commandable.give_pure_move_order(order);
}

fn process_attack_move(
    command_core: CommandCore,
    context: &mut ResMut<PlayerUnitCommander>,
    q: &mut Query<&mut Commandable>,
){
    let commandable = q.get_mut(command_core.recieving_unit);
    let mut commandable = commandable.unwrap();
    let commandable = commandable.as_mut();

    let attack_move = context.attack_moves.pop();
    let attack_move = attack_move.unwrap();
    let order = AttackMoveOrder{
        waypoint: attack_move.waypoint,
    };

    commandable.give_attack_move_order(order);
}

fn process_attack_target(
    command_core: CommandCore,
    context: &mut ResMut<PlayerUnitCommander>,
    q: &mut Query<&mut Commandable>,
){
    let commandable = q.get_mut(command_core.recieving_unit);
    let mut commandable = commandable.unwrap();
    let commandable = commandable.as_mut();

    let attack_target = context.attack_targets.pop();
    let attack_target = attack_target.unwrap();
    let order = AttackTargetOrder {
        invalidated: false,
        target_unit: attack_target.target_unit,
    };

    commandable.give_attack_target_order(order);
}