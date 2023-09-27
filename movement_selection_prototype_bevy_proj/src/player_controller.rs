use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing player_controller.rs");
        app
           .add_systems(Update, update);
    }
}

fn update(){

}