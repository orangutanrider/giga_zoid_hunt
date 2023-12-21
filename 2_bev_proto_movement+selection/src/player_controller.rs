mod mouse_controls;
pub mod selection_controller;
pub mod control_group_controller;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing player_controller.rs");
        app
            .add_plugins((
                mouse_controls::InitializePlugin,
                selection_controller::InitializePlugin,
            ))
            .add_systems(Update, update);
    }
}

fn update(){

}