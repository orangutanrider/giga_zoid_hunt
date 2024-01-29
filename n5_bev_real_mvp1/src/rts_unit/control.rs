pub mod commander;
pub mod selector;
pub mod prelude;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            selector::InitializePlugin,
        ));
    }
}