pub mod unit_selection;
pub mod unit_selection_proto;
mod unit_commands;
pub mod rapier_mouse;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller");
        app.add_plugins((
            unit_selection::InitializePlugin,
            unit_commands::InitializePlugin,
        ));
    }
}
