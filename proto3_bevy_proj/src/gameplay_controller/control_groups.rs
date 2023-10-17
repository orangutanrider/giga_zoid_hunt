use bevy::prelude::*;

use crate::unit::*;

#[derive(Component)] 
pub struct ControlGroup{
    index: u8,
    units: Vec<Entity>,
    entity: Entity,
}

#[derive(Clone, Copy)]
pub struct AddUnitRequest{
    pub group_index: u8,
    pub unit_entity: Entity,
}

#[derive(Component)]
pub struct AddUnitToGroupManager{
    requests: Vec<AddUnitRequest>, // new units that're being added to the group
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing gameplay_controller::control_groups");
        app
            .add_systems(Startup, (
                spawn_startup_control_groups,
            ))
            .add_systems(PostUpdate, process_add_unit_requests);
    }
}

// Startup
fn spawn_startup_control_groups(mut commands: Commands){
    let mut i = 0;

    while i < 9 {
        spawn_new_control_group(&mut commands, i);
        i = i + 1;
    }
}

// Callback processing
fn process_add_unit_requests(
    shift_input: Res<Input<KeyCode>>,

    mut manager_q: Query<&mut AddUnitToGroupManager>,
    mut group_q_mut: Query<&mut ControlGroup>,
    mut selectable_q: Query<&mut Selectable>,
    group_q: Query<&ControlGroup>,
) {
    let mut manager = manager_q.single_mut();

    if manager.requests.len() == 0 {
        return;
    }

    // Store current control group
    // This system works best if requests are bunched together, by the control group they're wanting be added to
    let mut curr_index = manager.requests[0].group_index; 
    let curr_entity = get_control_group_by_index(&group_q, curr_index);

    if curr_entity == None { // Error, clear manager and return
        manager.requests.clear();
        return;
    }

    let mut curr_entity = curr_entity.unwrap();
    let mut curr_group = group_q_mut.get_mut(curr_entity).unwrap();

    let shift_pressed = shift_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if !shift_pressed { 
        clear_control_group(&mut selectable_q, &mut curr_group);
    }

    for request in manager.requests.iter(){
        // If new group
        if request.group_index != curr_index { 
            // Store it
            curr_index = request.group_index;
            let curr_entity_unchecked = get_control_group_by_index(&group_q, curr_index);

            if curr_entity_unchecked == None { // Error, clear manager and return
                manager.requests.clear();
                return;
            }

            curr_entity = curr_entity_unchecked.unwrap();
            curr_group = group_q_mut.get_mut(curr_entity).unwrap();

            if !shift_pressed {
                // If the requests are given in a [1,2,1] order, this will break
                // Cause it'll clear on 2, removing one, and then adding the other one
                clear_control_group(&mut selectable_q, &mut curr_group);
            }
        }

        // Is the unit already in this control group?
        let mut unit_selectable = selectable_q.get_mut(request.unit_entity).unwrap();
        if unit_selectable.in_control_groups[curr_index as usize] == true {
            continue; // Then don't add them to it again
        }

        // Store data on the unit that tells this system that the unit is in a control group of the current index
        let unit_selectable = unit_selectable.as_mut();
        unit_selectable.in_control_groups[curr_index as usize] = true;

        curr_group.units.push(request.unit_entity); // add unit to control group
    }

    manager.requests.clear();
}

// Update
fn select_control_group_input(

){

}

// Internal
fn clear_control_group(
    selectable_q: &mut Query<&mut Selectable>,
    control_group: &mut ControlGroup,
){
    for unit in control_group.units.iter_mut(){
        let mut selectable = selectable_q.get_mut(*unit).unwrap();
        let selectable = selectable.as_mut();
        selectable.in_control_groups[control_group.index as usize] = false;
    }
    control_group.units.clear();
}

fn spawn_new_control_group( // This function does not handle the error case where you try to create a control group with an index that another group already has
    commands: &mut Commands,
    index: u8,
) {
    let mut spawn = commands.spawn_empty();

    let entity = spawn.id();

    spawn.insert(
        ControlGroup{
            index,
            units: Vec::new(),
            entity,
        }
    );
}

fn get_control_group_by_index(
    q: &Query<&ControlGroup>,
    index: u8,
) -> Option<Entity> {
    if index > 9 {
        println!("Control groups are limited to 0-9");
        return  None;
    }

    for group in q.iter() {
        if group.index != index {
            continue;
        }

        return Some(group.entity.clone());
    }

    return None;
}

// Callbacks
pub fn add_unit_to_control_group(
    manager: &mut AddUnitToGroupManager,
    group_index: u8,
    unit_entity: Entity,
){
    manager.requests.push(
        AddUnitRequest{group_index, unit_entity}
    );
}
