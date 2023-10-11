use bevy::prelude::*;
use crate::unit::spawning::*;
use crate::unit::UnitEntity;
use crate::gameplay_controller::selection::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing proto_builder.rs");
        app
           .add_systems(Startup, spawn_units)
           .add_systems(Update, (
            print_unit_entity_values_on_input_p,
            print_selection_on_input_o,
        ))
           ;
    }
}

fn spawn_units(mut q: Query<&mut UnitSpawnManager>) {
    let manager =  &mut q.single_mut();

    spawn_unit(manager, UnitSpawnRequest{spawn_location: Vec3{x: 0.0, y: 0.0, z: 0.0}});
    spawn_unit(manager, UnitSpawnRequest{spawn_location: Vec3{x: 200.0, y: 0.0, z: 0.0}});
    spawn_unit(manager, UnitSpawnRequest{spawn_location: Vec3{x: -200.0, y: 0.0, z: 0.0}});
    spawn_unit(manager, UnitSpawnRequest{spawn_location: Vec3{x: 0.0, y: 200.0, z: 0.0}});
    spawn_unit(manager, UnitSpawnRequest{spawn_location: Vec3{x: 0.0, y: -200.0, z: 0.0}});
}

fn print_unit_entity_values_on_input_p(
    q: Query<&mut UnitEntity>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::P){
        return;
    }

    println!("");
    println!("print_unit_entity_values_on_input_p");

    for unit in q.iter(){
        println!("{}", unit.0.index());
    }
}

fn print_selection_on_input_o(
    input: Res<Input<KeyCode>>,
    q: Query<&mut UnitSelection>,
) {
    if !input.just_pressed(KeyCode::O){
        return;
    }

    println!("");
    println!("print_selection_on_input_o");

    let selection = q.single();
    
    for selected in selection.selection.iter(){
        println!("{}", selected.index());
    }
}