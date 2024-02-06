pub mod autonomous;
pub mod controlled;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(controlled::InitializePlugin);
    }
}