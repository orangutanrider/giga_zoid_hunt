use bevy::prelude::*;

use crate::gameplay_controller::selection::*;
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing gameplay_controller::control_groups::input");
        app
            .add_systems(PostUpdate, (
                select_control_group_input,
                add_selection_to_control_group_input,
            ));
    }
}

// Update
fn add_selection_to_control_group_input(
    keys_input: Res<Input<KeyCode>>,
    selection_q: Query<&mut UnitSelection>,
    mut manager_q: Query<&mut AddUnitToGroupManager>,
){
    if !keys_input.pressed(KeyCode::ControlLeft){
        return;
    }

    if keys_input.just_pressed(KeyCode::Key1) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 1);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key2) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 2);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key3) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 3);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key4) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 4);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key5) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 5);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key6) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 6);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key7) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 7);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key8) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 8);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key9) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 9);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key0) {
        let selection = selection_q.single();
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        add_selection_to_control_group(& selection, manager, 0);
        return;
    }
}

fn select_control_group_input(
    keys_input: Res<Input<KeyCode>>,
    mut group_q: Query<&mut ControlGroup>,
    mut manager_q: Query<&mut NewSelectionManager>,
    mut unit_q: Query<&mut UnitEntity>,
){
    if keys_input.pressed(KeyCode::ControlLeft){
        return;
    }

    if keys_input.just_pressed(KeyCode::Key1) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 1).unwrap();

        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key2) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 2).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key3) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 3).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key4) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 4).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key5) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 5).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key6) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 6).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key7) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 7).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key8) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 8).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key9) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 9).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }

    if keys_input.just_pressed(KeyCode::Key0) {
        let mut manager = manager_q.single_mut();
        let manager = manager.as_mut();

        let entity = get_control_group_by_index(&group_q, 0).unwrap();
        
        let mut group = group_q.get_mut(entity).unwrap();
        let group = group.as_mut();
        select_control_group(&mut unit_q, manager, group);
        return;
    }
}

// Internal
fn add_selection_to_control_group(
    selection: & UnitSelection,
    manager: &mut AddUnitToGroupManager,
    group_index: u8
) {
    for unit_entity in selection.selection.iter(){
        add_unit_to_control_group(manager, group_index, *unit_entity);
    }
}

fn select_control_group(
    q: &mut Query<&mut UnitEntity>,
    manager: &mut NewSelectionManager,
    group: &mut ControlGroup,
){
    for entity in group.units.iter() {
        let mut unit_entity = q.get_mut(*entity).unwrap(); // Having to do this is kinda crappy
        let unit_entity = unit_entity.as_mut(); 

        select(manager, unit_entity); // It'd be better if I could just give this function the entity
    }
}

pub fn get_control_group_by_index(
    q: &Query<&mut ControlGroup>,
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