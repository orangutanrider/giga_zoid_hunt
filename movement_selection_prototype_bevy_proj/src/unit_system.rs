pub mod unit_spawning;

use bevy::prelude::*;

#[derive(Component)]
pub struct Unit{
    // waypoint list
    pub entity_index: u32,
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit_system.rs");
        app
           .add_plugins(unit_spawning::InitializePlugin)
           .add_systems(Update, update);
    }
}

fn update(){

}