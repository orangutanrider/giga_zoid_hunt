mod unit_spawning;

use bevy::prelude::*;

#[derive(Component)]
struct Unit{
    // waypoint list
}

#[derive(Bundle)]
struct UnitBundle{
    pub unit: Unit,
    pub sprite_bundle: SpriteBundle,
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