pub mod spawning;

use bevy::prelude::*;

#[derive(Component)]
pub struct Unit{

}

#[derive(Component)]
pub struct UnitEntity(pub Entity);

#[derive(Component)]
pub struct Selectable{
    pub is_selected: bool,
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