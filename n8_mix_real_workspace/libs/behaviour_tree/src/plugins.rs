//! If you want to define your own plugins, go to internal_systems.

pub mod internal_systems;

use bevy::prelude::*;

pub use crate::state::StatePlugin;
pub use crate::bang::BangPlugin;
pub use crate::root::RootPlugin;

pub struct AllPlugins;
impl Plugin for AllPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            StatePlugin,
            BangPlugin,
            RootPlugin,
        ));
    }
}