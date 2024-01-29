use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::RtsCommanderContext;
use crate::rts_unit::control::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, command_pure_move)
    }
}

#[derive(SystemParam)]
struct PureMoveInput<'w> {
    keys: Res<'w, Input<KeyCode>>,
}
impl<'w> PureMoveInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::D];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

/// Update system
fn command_pure_move (
    pure_move_input: PureMoveInput, 
    mut commander_context: RtsCommanderContext,
) {
    if !pure_move_input.just_pressed() {
        return;
    }
    
    let unit_commander = &mut commander_context.unit_commander;
    let mouse = &commander_context.mouse;
    let add_mode = commander_context.add_mode_input.is_pressed();
    let order = OrderType::PureMovement(
        PureMovementOrder{waypoint: commander_context.mouse.position()}
    );
    unit_commander.command_selection(add_mode, order)
}