pub mod spawning;

use bevy::prelude::*;

#[derive(Component)]
pub struct Unit{

}

#[derive(Component)]
pub struct UnitEntity(pub Entity);

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