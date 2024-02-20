use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::RtsCommanderContext;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, command_stop);
    }
}

#[derive(SystemParam)]
struct StopInput<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
}
impl<'w> StopInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::KeyS];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

/// Update system
fn command_stop (
    stop_input: StopInput, 
    mut commander_context: RtsCommanderContext,
) {
    if !stop_input.just_pressed() {
        return;
    }
    
    let unit_commander = &mut commander_context.unit_commander;
    unit_commander.command_selection_stop()
}