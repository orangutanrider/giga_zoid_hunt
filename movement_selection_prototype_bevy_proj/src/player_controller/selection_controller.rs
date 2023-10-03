use bevy::prelude::*;
use crate::unit_system::Unit;

#[derive(Component)]
pub struct UnitSelectionManager{
    unit_ids: Vec<u128>,
}

#[derive(Component)]
pub struct UnitSelection{
    unit_ids: Vec<u128>,
}

#[derive(Component)]
pub struct ControlGroup{
    // possibly add a keycode value in the future
    index: u8,
    unit_ids: Vec<u128>,
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing selection_controller.rs");
        app
            .add_systems(Startup, (
                spawn_selection_manager,
                spawn_selected_group,
                spawn_control_groups,
            ))
            .add_systems(PostUpdate, update);
    }
}

// Startup
fn spawn_selection_manager(mut commands: Commands){
    commands.spawn(UnitSelectionManager{
        unit_ids: Vec::new()
    });
}

fn spawn_selected_group(mut commands: Commands) {
    commands.spawn(UnitSelection{
        unit_ids: Vec::new()
    });
}

fn spawn_control_groups(mut commands: Commands) {
    // 1 - 10 (0 - 9)
    for i in 0..9 {
        commands.spawn(ControlGroup{
            index: i,
            unit_ids: Vec::new()
        });
    }
}

// Callback Processing
fn update(
    input: Res<Input<KeyCode>>,
    mut manager_q: Query<&mut UnitSelectionManager>,
    mut selection_q: Query<&mut UnitSelection>,
) {
    let manager = &mut manager_q.single_mut();
    if manager.unit_ids.len() == 0 {return;}

    let selection = &mut selection_q.single_mut();

    if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        selection.unit_ids.clear();
    }

    for unit in manager.unit_ids.iter_mut(){
        selection.unit_ids.push(*unit);
    }

    manager.unit_ids.clear();
}

// Callbacks
pub fn select_unit(
    manager: &mut UnitSelectionManager, 
    unit: Unit,
) {
    manager.unit_ids.push(unit.id);
}

pub fn select_control_group(
    unit_selection: &mut UnitSelection,
    control_group: &mut ControlGroup,
) {
    for unit in control_group.unit_ids.iter_mut(){
        unit_selection.unit_ids.push(*unit);
    }
}

pub fn add_selection_to_control_group(
    selection: &mut UnitSelection, 
    control_group: &mut ControlGroup,
) {
    for unit in selection.unit_ids.iter_mut(){
        control_group.unit_ids.push(*unit);
    }
}