pub mod spawning;
pub mod movement;

use bevy::prelude::*;

#[derive(Component)]
pub struct Unit{

}

// Refactor this and put the entity value into the Unit struct
// Look to the control groups dynamic implementation in proto_src for an example of this
#[derive(Component)]
pub struct UnitEntity(pub Entity);

#[derive(Component)]
pub struct Selectable{
    pub is_selected: bool,
    pub in_control_groups: [bool; 10],
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit.rs");
        app
            .add_plugins((
                spawning::InitializePlugin,
                movement::InitializePlugin,
            ));
    }
}