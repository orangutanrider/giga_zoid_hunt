/// Handles unit selection functions, does nothing on its own

mod mouse_input;

use bevy::prelude::*;
use crate::unit::*;
use crate::unit::selectable::*;
use bevy::ecs::system::SystemParam;
use std::slice::Iter;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller::selection");
        app
        .add_plugins(mouse_input::InitializePlugin)
        .init_resource::<SelectedUnits>();
    }
}

#[derive(Clone, Copy)]
pub struct SelectionPush {
    push_type: SelectionPushType,
    selectable_unit: UnitID,
}

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum SelectionPushType {
    PlayerInputMarker,
    EntityPush,
}

#[derive(SystemParam)]
pub struct UnitSelectionCommands<'w, 's>{
    input: Res<'w, Input<KeyCode>>,
    context: ResMut<'w, SelectedUnits>,
    q: Query<'w, 's, &'static mut Selectable>,
}
impl<'w, 's> UnitSelectionCommands<'w, 's> {
    pub fn empty_select_input(&mut self) {
        let context = &mut self.context;
        let input = & self.input;

        // If shift isn't held, clear selection when an attempted selection happens
        if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            self.clear_selection();
        }
    }

    pub fn select_unit(&mut self, unit_id: UnitID) {
        let context = &mut self.context;
        let input = & self.input;
        let q = &mut self.q;

        // If shift isn't held, clear selection when new selections arrive
        if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            self.clear_selection();
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
        context.selected.push(unit_id);
    }

    pub fn clear_selection(&mut self){
        let context = &mut self.context;
        context.selected.clear();
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
}