/// Handles unit selection functions, does nothing on its own

pub mod input;

use bevy::prelude::*;
use crate::unit::*;
use crate::unit::selectable::*;
use bevy::ecs::system::SystemParam;
use std::slice::Iter;

use super::add_mode::AddModeInput;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller::selection");
        app
        .add_plugins(input::InitializePlugin)
        .init_resource::<SelectedUnits>();
    }
}

#[derive(SystemParam)]
pub struct UnitSelectionCommands<'w, 's>{
    add_mode: AddModeInput<'w>,
    selected: ResMut<'w, SelectedUnits>,
    selectable: ParamSet<'w, 's, (
        Query<'w, 's, &'static mut Selectable>,
        Query<'w, 's, &'static Selectable>,
    )>
}
impl<'w, 's> UnitSelectionCommands<'w, 's> {
    pub fn select_input(&mut self) {
        if !self.add_mode.is_pressed() {
            self.clear_selection();
        }
    }

    pub fn select_unit(&mut self, unit_id: UnitID) {
        let selected = &mut self.selected;
        let q = &mut self.selectable.p0();

        // If shift isn't held, clear selection when new selections arrive
        if !self.add_mode.is_pressed() {
            selected.clear();
        }
        
        // Get selectable
        let selectable = q.get_mut(unit_id.0);
        let mut selectable = selectable.unwrap();

        // Don't add already selected units to selection
        if selectable.is_selected == true {
            return;
        }

        // Set selected true
        let selectable = selectable.as_mut();
        selectable.is_selected = true;

        // Add to selection
        selected.selected.push(unit_id);
    }

    pub fn clear_selection(&mut self){
        self.selected.clear();
    }

    pub fn selected_iter(&self) -> Iter<'_, UnitID> {
        return self.selected.iter();
    }
}

#[derive(Resource, Default)]
pub struct SelectedUnits {
    selected: Vec<UnitID>,
}
impl SelectedUnits {
    pub fn iter(&self) -> Iter<'_, UnitID> {
        return self.selected.iter();
    }

    fn clear(&mut self) {
        self.selected.clear();
    }
}