use bevy::prelude::*;

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