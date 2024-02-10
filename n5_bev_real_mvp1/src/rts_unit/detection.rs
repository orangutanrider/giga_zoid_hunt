mod single_result_types;
mod detectors;

pub mod blocks;
pub mod parts;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            detectors::InitializePlugin,
            single_result_types::InitializePlugin,
        ));
    }
}
