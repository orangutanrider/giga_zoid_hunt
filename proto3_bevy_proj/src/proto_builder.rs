use bevy::prelude::*;
use crate::unit::spawning::*;
use crate::unit::UnitEntity;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing proto_builder.rs");
        app
           .add_systems(Startup, spawn_units)
           //.add_systems(Update, print_unit_entity_values)
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

fn print_unit_entity_values(q: Query<&mut UnitEntity>){
    println!("");
    println!("Unit entity values:");

    for unit in q.iter(){
        println!("{}", unit.0.index());
    }
}