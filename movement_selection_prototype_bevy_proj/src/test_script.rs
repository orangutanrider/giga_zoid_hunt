use bevy::prelude::*;
use crate::unit_system::unit_spawning::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing test_script.rs");
        app
           .add_systems(PostStartup, spawn_test_units);
    }
}

fn spawn_test_units(mut q: Query<&mut UnitSpawnList>){
    let spawn_list =  &mut q.single_mut();

    spawn_unit(Vec3{x: 0.0, y: 0.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: 200.0, y: 0.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: -200.0, y: 0.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: 0.0, y: 200.0, z: 0.0}, spawn_list);
    spawn_unit(Vec3{x: 0.0, y: -200.0, z: 0.0}, spawn_list);
}