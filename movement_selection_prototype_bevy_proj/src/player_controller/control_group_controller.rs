use bevy::prelude::*;
use super::selection_controller::*;
use crate::unit_system::*;

#[derive(Component)]
pub struct ControlGroupManager{
    group_unit_id_data: Vec<(u8, (u128, SelectionData))>,
}

#[derive(Component)]
pub struct ControlGroup{
    // possibly add a keycode value in the future
    index: u8,
    unit_ids: Vec<u128>,
}
impl ControlGroup {
    pub const INVALID_GROUP: u8 = u8::MAX;
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing selection_controller.rs");
        app
            .add_systems(Startup, (
                spawn_control_group_manager,
                spawn_base_control_groups,
            ))
            .add_systems(PostUpdate, process_control_group_assignations);
    }
}

// Startup
fn spawn_control_group_manager(mut commands: Commands){
    commands.spawn(ControlGroupManager{
        group_unit_id_data: Vec::new()
    }); 
}


fn spawn_base_control_groups(mut commands: Commands) {
    // 1 - 10 (0 - 9)
    for i in 0..9 {
        commands.spawn(ControlGroup{
            index: i,
            unit_ids: Vec::new()
        });
    }
}

// Callback Processing
fn process_control_group_assignations(
    input: Res<Input<KeyCode>>,
    mut manager_q: Query<&mut ControlGroupManager>,
    mut group_q: Query<&mut ControlGroup>,
) {

}

// Internal
fn try_add_unit_to_control_group(
    control_group: &mut ControlGroup,
    unit: &mut Unit,
) -> bool {
    let group_id = control_group.index;
    for in_group in unit.selection_data.in_control_groups.iter(){ // check if already in group
        if *in_group == group_id {return false;}
    }

    return true;
}

// Callbacks
pub fn add_selection_to_control_group(
    selection: &mut UnitSelection, 
    control_group: &mut ControlGroup,
) {
    for unit in selection.unit_id_data.iter_mut(){
        control_group.unit_ids.push(*unit);
    }
}

pub fn select_control_group(
    unit_selection: &mut UnitSelection,
    control_group: &mut ControlGroup,
) {
    for unit in control_group.unit_ids.iter_mut(){
        //unit_selection.unit_ids.push(*unit);
    }
}