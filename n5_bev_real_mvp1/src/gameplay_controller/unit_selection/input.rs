
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::*;
use crate::gameplay_controller::unit_mouse::*;

use mouse_tracking::MousePosWorld;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller::selection::mouse");
        app.add_systems(Update, select_units);
    }
}

#[derive(SystemParam)]
pub struct SelectInput<'w> {
    mouse_buttons: Res<'w, Input<MouseButton>>,
}
impl<'w> SelectInput<'w> {
    const BUTTONS: [MouseButton; 1] = [MouseButton::Left];

    pub fn just_pressed(&self) -> bool {
        return self.mouse_buttons.any_just_pressed(Self::BUTTONS);
    }
    
    pub fn just_released(&self) -> bool {
        return self.mouse_buttons.any_just_released(Self::BUTTONS);
    }
}

/// Update function
fn select_units(
    mouse: UnitMouse,
    input: SelectInput,
    mut selection_commands: UnitSelectionCommands,
){
    if !input.just_released() {
        return;
    }
    selection_commands.select_input();

    let units = mouse.selection_drag_click_release();
    for unit_id in units.iter() {
        selection_commands.select_unit(*unit_id);
    }
}