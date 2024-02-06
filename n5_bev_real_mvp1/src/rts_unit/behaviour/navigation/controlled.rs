pub mod basic_controlled_navigation;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(basic_controlled_navigation::InitializePlugin);
    }
}