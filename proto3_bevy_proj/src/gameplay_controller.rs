mod mouse;
pub mod selection;
pub mod unit_orders;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing player_controller.rs");
        app
            .add_plugins((
                mouse::InitializePlugin,
                selection::InitializePlugin,
                unit_orders::InitializePlugin,
            ));
    }
}
