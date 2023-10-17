// This was a version of the file where it'd dynamically create new control groups whenver it recieved an index of one that didn't currently exist
// I dropped this implementation because it was too much
// Bevy doesn't immediatly spawn in objects when you use the spawn command, so there'd have to be a system for adding requests to a seperate list
// So it could wait for the group to be spawned and add them to it
// It'd be too much work and complexity for something that doesn't really matter
// So I'm going with a fixed number of control groups implementation

use bevy::prelude::*;

use crate::unit::*;

#[derive(Component)]
pub struct ControlGroup{
    pub index: u8,
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

// Startup
pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing gameplay_controller::control_groups");
        /* 
        app
            .add_systems(Startup, (
                spawn_new_selection_manager,
                spawn_selection,
            ))
            .add_systems(PostUpdate, process_selection_requests);
        */
    }
}

// Callback processing
fn process_add_unit_requests(
    shift_input: Res<Input<KeyCode>>,

    mut manager_q: Query<&mut AddUnitToGroupManager>,
    groups_q: Query<&ControlGroup>,
    mut groups_q_mut: Query<&mut ControlGroup>,
) {
    let manager = manager_q.single_mut();

    if manager.requests.len() == 0 {
        return;
    }

    let mut curr_index = manager.requests[0].group_index;
    let mut curr_entity = get_control_group_by_index(&groups_q, curr_index).unwrap();
    let mut curr_group = groups_q_mut.get_mut(curr_entity).unwrap();

    for request in manager.requests.iter(){
        if request.group_index != curr_index {
            // add requests to a seperate list
        }

    }

    if !shift_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]){

    }
}

// Update
fn select_control_group_input(

){

}

// Internal
fn spawn_new_control_group( // This function does not handle the error case where you try to create a control group with an index that another group already has
    mut commands: Commands,
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
