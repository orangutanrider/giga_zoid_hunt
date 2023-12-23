use bevy::prelude::*;
use crate::unit_system::*;

#[derive(Component)]
pub struct UnitSelectionManager{
    unit_ids: Vec<u32>,
}

#[derive(Component)]
pub struct UnitSelection{
    pub unit_id_data: Vec<u32>,
}
/* 
impl UnitSelection{
    fn new() -> UnitSelection {
        UnitSelection{unit_id_data: Vec::new()}
    }
}
impl Iterator for UnitSelection{
    type Item = usize;

    fn next(&mut self) -> Option<Unit> {
        self.count += 1;
        
    }
}
*/

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing selection_controller.rs");
        app
            .add_systems(Startup, (
                spawn_selection_manager,
                spawn_unit_selection,
            ))
            .add_systems(PostUpdate, process_new_selections);
    }
}

// Startup
fn spawn_selection_manager(mut commands: Commands){
    commands.spawn(UnitSelectionManager{
        unit_ids: Vec::new()
    });
}

fn spawn_unit_selection(mut commands: Commands) {
    commands.spawn(UnitSelection{
        unit_id_data: Vec::new()
    });
}

// Callback Processing
fn process_new_selections(
    input: Res<Input<KeyCode>>,
    mut manager_q: Query<&mut UnitSelectionManager>,
    mut selection_q: Query<&mut UnitSelection>,
) {
    let manager = &mut manager_q.single_mut();
    if manager.unit_ids.len() == 0 {return;}

    let selection = &mut selection_q.single_mut();

    // if shift isn't held, clear selection
    if !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        selection.unit_id_data.clear();
    }

    // add units to selection
    for id in manager.unit_ids.iter_mut(){
        selection.unit_id_data.push(*id);
    }

    manager.unit_ids.clear();
}

// Internal
fn try_add_unit_to_selection(
    manager: &mut UnitSelectionManager, 
    unit: &Unit,
) -> bool {
    manager.unit_ids.push(unit.entity_index);
    return true;
}

// Callbacks
pub fn select_unit(
    manager: &mut UnitSelectionManager, 
    unit: &Unit,
) {
    try_add_unit_to_selection(manager, &unit);
    
}

/*
struct Counter{
    count: usize,
}
impl Counter{
    fn new() -> Counter {
        Counter{count: 0}
    }
}

impl Iterator for Counter{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        
    }
}
*/