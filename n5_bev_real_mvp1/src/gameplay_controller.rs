pub mod selection;
pub mod rapier_mouse;
mod selection_commands;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller");
    }
}
