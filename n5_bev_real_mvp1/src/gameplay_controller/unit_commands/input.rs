// I suppose the mod tree for this and commands should be the otherway around, if refactor, I'll do that, naming conventions too, for both scripts

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::gameplay_controller::unit_mouse::UnitMouse;

use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            order_attack,
            order_move,
            order_stop,
        ));
    }
}

#[derive(SystemParam)]
struct AttackInput<'w> {
    keys: Res<'w, Input<KeyCode>>,
}
impl<'w> AttackInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::A];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

#[derive(SystemParam)]
struct MoveInput<'w> {
    keys: Res<'w, Input<KeyCode>>,
}
impl<'w> MoveInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::D];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

#[derive(SystemParam)]
struct StopInput<'w> {
    keys: Res<'w, Input<KeyCode>>,
}
impl<'w> StopInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::S];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

#[derive(SystemParam)]
struct CommanderContext<'w, 's> {
    order_commands: OrderUnitCommands<'w, 's>,
    selection_commands: UnitSelectionCommands<'w, 's>,
    mouse: UnitMouse<'w, 's>,
}

/// Update system
fn order_attack (
   attack_input: AttackInput, 
   mut commander_context: CommanderContext,
) {
    if !attack_input.just_pressed() {
        return;
    }

    let order_commands = &mut commander_context.order_commands;
    let selection_commands = & commander_context.selection_commands;
    let mouse = & commander_context.mouse;
    let waypoint = mouse.mouse_location();
    //order_commands.command_attack(waypoint, & selection_commands); // replace with this eventually
    order_commands.command_attack_move(waypoint, & selection_commands);
}

/// Update system
fn order_move (
    move_input: MoveInput, 
    mut commander_context: CommanderContext,
) {
    if !move_input.just_pressed() {
        return;
    }
    
    let order_commands = &mut commander_context.order_commands;
    let selection_commands = & commander_context.selection_commands;
    let mouse = & commander_context.mouse;
    let waypoint = mouse.mouse_location();
    order_commands.command_pure_move(waypoint, & selection_commands);
}

/// Update system
fn order_stop (
    stop_input: StopInput, 
    mut commander_context: CommanderContext,
) {
    if !stop_input.just_pressed() {
        return;
    }
    
    let order_commands = &mut commander_context.order_commands;
    let selection_commands = & commander_context.selection_commands;
    order_commands.command_stop(& selection_commands)
}