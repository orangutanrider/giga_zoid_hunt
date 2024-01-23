
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
    unit_mouse: UnitMouse,
    input: SelectInput,
    mut selection_commands: UnitSelectionCommands,
){
    if !input.just_released() {
        return;
    }
    selection_commands.select_input();

    let selectable_q = & selection_commands.selectable.p1();
    let units = selection_drag_click_release(&unit_mouse, selectable_q);
    for unit_id in units.iter() {
        selection_commands.select_unit(*unit_id);
    }
}

fn selection_drag_click_release(
    unit_mouse: &UnitMouse,
    selectable_q: &Query<&Selectable>
) -> Vec<UnitID> {
    let mut return_vec: Vec<UnitID> = Vec::new();
    let callback = |entity| -> bool {
        let selectable = selectable_q.get(entity);
        if selectable.is_err() {
            return false;
        }
        return_vec.push(UnitID(entity));
        return true;
    };
    let location1 = unit_mouse.mouse_down_origin();
    let location2 = unit_mouse.mouse_location();
    unit_mouse.box_intersect(location1, location2, callback);

    return return_vec;
}