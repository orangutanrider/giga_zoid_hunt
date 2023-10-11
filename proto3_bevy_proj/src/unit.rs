pub mod spawning;

use bevy::prelude::*;

#[derive(Component)]
pub struct Unit{

}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit.rs");
        app
            .add_plugins((
                spawning::InitializePlugin,
            ));
    }
}