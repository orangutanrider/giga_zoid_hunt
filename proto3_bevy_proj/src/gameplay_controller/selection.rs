use bevy::prelude::*;

use crate::unit::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing selection.rs");
        app
            .add_systems(Startup, (
                spawn_new_selection_manager,
                spawn_selection,
            ))
            .add_systems(PostUpdate, process_selection_requests);
    }
}

#[derive(Component)]
pub struct UnitSelection{
    pub selection: Vec<Entity>, // See if it is possible to make the Vec public read-only but have the Entities public editable
}

#[derive(Component)]
pub struct NewSelectionManager{
    selection_requests: Vec<SelectionRequest>
}

#[derive(Clone, Copy)]
pub struct SelectionRequest{
    entity: Entity,
}

// Startup
fn spawn_new_selection_manager(mut commands: Commands){
    commands.spawn(NewSelectionManager{
        selection_requests: Vec::new(),
    });
}

fn spawn_selection(mut commands: Commands){
    commands.spawn(UnitSelection{
        selection: Vec::new(),
    });
}

// Callback processing
fn process_selection_requests(
    input: Res<Input<KeyCode>>,
    mut manager_q: Query<&mut NewSelectionManager>,
    mut selection_q: Query<&mut UnitSelection>,
    mut selectable_q: Query<&mut Selectable>,
){
    let mut manager = manager_q.single_mut();

    if manager.selection_requests.len() == 0 {
        return;
    }

    let mut selection = selection_q.single_mut();

    // Replace with input package in future
    if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]){
        let selection = selection.as_mut();
        clear_selection(&mut selectable_q, selection);
    }

    for selection_request in manager.selection_requests.iter(){
        // set selected true
        let selectable = selectable_q.get_mut(selection_request.entity);
        let mut selectable = selectable.unwrap();

        if selectable.is_selected == true {
            continue;
        }

        let selectable = selectable.as_mut();
        selectable.is_selected = true;

        // add to selection
        selection.selection.push(selection_request.entity);
    }

    manager.selection_requests.clear();
}

// Internal
fn clear_selection(
    q: &mut Query<&mut Selectable>,
    selection: &mut UnitSelection,
){
    println!("Clear selection");

    for entity in selection.selection.iter_mut(){
        let selectable = q.get_mut(*entity);
        let mut selectable = selectable.unwrap();
        let selectable = selectable.as_mut();
        selectable.is_selected = false
    }

    selection.selection.clear();
}

// Callbacks
pub fn select(
    manager: &mut NewSelectionManager, 
    selectable: &Selectable, 
    unit_entity: &UnitEntity
) {
    manager.selection_requests.push(SelectionRequest { 
        entity: unit_entity.0, 
    });

    println!("Tried to select");
}