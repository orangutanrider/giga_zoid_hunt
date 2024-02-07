pub mod hunt_prince;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(hunt_prince::InitializePlugin);
    }
}