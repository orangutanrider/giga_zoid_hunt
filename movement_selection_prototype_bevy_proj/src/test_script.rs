use bevy::prelude::*;
use crate::unit_system::*;
use crate::unit_system::unit_spawning::*;
use crate::player_controller::selection_controller::*;
use crate::player_controller::control_group_controller::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing test_script.rs");
        app
           .add_systems(PostStartup, spawn_test_units)
           .add_systems(Update, (
            print_selection_on_input,
            print_all_unit_ids_on_input,
        ));
    }
}

fn spawn_test_units(mut q: Query<&mut UnitSpawnManager>){
    let spawn_list =  &mut q.single_mut();

    spawn_unit(Vec3{x: 0.0, y: 0.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: 200.0, y: 0.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: -200.0, y: 0.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: 0.0, y: 200.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: 0.0, y: -200.0, z: 0.0}, spawn_list);
}

fn print_all_unit_ids_on_input(
    keys: Res<Input<KeyCode>>,
    unit_q: Query<&mut Unit>
){
    if !keys.just_pressed(KeyCode::L) {return;}

    println!("");
    println!("TEST: print_all_unit_ids_on_input");
    for unit in unit_q.iter(){
        println!("unit with ID: {} exists", unit.entity_index);
    }
}


fn print_selection_on_input(
    keys: Res<Input<KeyCode>>,
    selection_q: Query<&mut UnitSelection>
){
    if !keys.just_pressed(KeyCode::K) {return;}

    println!("");
    println!("TEST: print_selection_on_input");
    let selection = selection_q.single();
    for id in selection.unit_id_data.iter(){
        println!("unit ID: {} is selected", id);
    }
}