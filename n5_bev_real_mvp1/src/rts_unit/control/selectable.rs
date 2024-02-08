pub mod selector;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            selector::InitializePlugin,
        ));
    }
}

#[derive(Component)]
pub struct Selectable {
    is_selected: bool,
}
impl Default for Selectable {
    fn default() -> Self {
        Self { 
            is_selected: false,
        }
    }
}

impl Selectable {
    pub fn new() -> Self {
        Self { 
            is_selected: false,
        }
    }
}