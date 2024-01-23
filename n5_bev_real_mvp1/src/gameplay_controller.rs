pub mod unit_selection;
mod unit_commands;
pub mod unit_mouse;
mod add_mode;

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
