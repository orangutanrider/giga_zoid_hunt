//! If you want to define your own plugins, go to tree_systems.

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