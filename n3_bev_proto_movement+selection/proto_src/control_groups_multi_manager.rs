// So I'd like to get this working
// The idea is to have a manager per control group
// But currently I don't know how to do that without just declaring a function and a manager struct per control group
// I'd guess it's possible to not have to do it that way though, atleast I hope so
// So I'm gonna look into the bevy Res<> and the bevy system params stuff in the future
// For now though I'll just finish off the current implementation for control groups, even if it isn't the best

use bevy::prelude::*;

use crate::unit::*;

#[derive(Component)] 
pub struct ControlGroup{
    index: u8,
    units: Vec<Entity>,
    entity: Entity,
}

#[derive(Component)]
pub struct ControlGroupManager{
    pub group_index: u8,
    new_units: Vec<Entity>, // units that're being added to the control group
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
            .add_systems(PostUpdate, process_manager_callbacks(0));
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
fn process_manager_callbacks(
    manager_index: u8,

    shift_input: Res<Input<KeyCode>>,

    mut manager_q: Query<&mut ControlGroupManager>,
    mut group_q_mut: Query<&mut ControlGroup>,
    group_q: Query<&ControlGroup>,

    mut selectable_q: Query<&mut Selectable>,
) {

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
