pub mod single_result_types;
pub mod circle_cast_detector;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            circle_cast_detector::InitializePlugin,
            single_result_types::InitializePlugin,
        ));
    }
}
