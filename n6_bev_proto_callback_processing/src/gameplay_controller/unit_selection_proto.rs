/// Handles selection callbacks and shift input

use bevy::prelude::*;
use crate::unit::*;
use crate::unit::selectable::*;
use bevy::ecs::system::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
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
pub struct SelectionCommand<'w, 's>{
    input: Res<'w, Input<KeyCode>>,
    context: ResMut<'w, SelectionContext>,
    q: Query<'w, 's, &'static mut Selectable>,
}
impl<'w, 's> SelectionCommand<'w, 's> {
    fn select_unit(&mut self, unit_id: UnitID) {
        let context = &mut self.context;
        let input = & self.input;
        let q = &mut self.q;

        // If shift isn't held, clear selection when new selections arrive
        if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            context.clear_selected(q);
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
}

#[derive(Resource, Default)]
pub struct SelectionContext {
    pub selected: Vec<UnitID>,
    pushed_selections: Vec<SelectionPush>,
}
impl SelectionContext {
    pub fn add_select(&mut self, unit: &Unit) {
        self.pushed_selections.push(SelectionPush { 
            push_type: SelectionPushType::EntityPush, 
            selectable_unit: unit.id, 
        });
    }
     
    pub fn mark_selection_input(&mut self) {
        self.pushed_selections.push(SelectionPush { 
            push_type: SelectionPushType::PlayerInputMarker, 
            selectable_unit: UnitID::PLACEHOLDER,
        });
    }

    fn clear_selected(&mut self, q: &mut Query<&mut Selectable>) {
        // Update component data
        for unit_id in self.selected.iter_mut() {
            let selectable = q.get_mut(unit_id.0);
            let mut selectable = selectable.unwrap();
            let selectable = selectable.as_mut();

            selectable.is_selected = false;
        }

        // Clear selected
        self.selected.clear();
    }
}

// Update system
fn process_selection_pushes(
    input: Res<Input<KeyCode>>, // Replace with input package/asset
    mut context: ResMut<SelectionContext>,
    mut q: Query<&mut Selectable>,
) {
    if context.pushed_selections.len() == 0 {
        return;
    }

    // If shift isn't held, clear selection when new selections arrive
    if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        context.clear_selected(&mut q);
    }

    // Process pushes, add to selected
    let pushes = context.pushed_selections.clone(); // Could potentially use two loops instead of doing this
    for selection_push in pushes.iter() {
        if selection_push.push_type == SelectionPushType::PlayerInputMarker{
            continue;
        }

        // Get selectable
        let selectable = q.get_mut(selection_push.selectable_unit.0);
        let mut selectable = selectable.unwrap();

        // Don't add already selected units to selection
        if selectable.is_selected == true {
            continue;
        }

        // Set selected true
        let selectable = selectable.as_mut();
        selectable.is_selected = true;

        // Add to selection
        context.selected.push(selection_push.selectable_unit);
    }

    // Clear pushes; they have now been processed
    context.pushed_selections.clear();
}
