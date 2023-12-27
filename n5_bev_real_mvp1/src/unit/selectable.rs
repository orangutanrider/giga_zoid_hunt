use bevy::prelude::*;

/// Attach to a unit, by adding it onto the entity that holds their UnitCoreBundle components
#[derive(Component)]
pub struct Selectable {
    // This data is stored to stop the same unit from being added to the selection or a control group multiple times
    pub is_selected: bool,
    pub in_control_groups: [bool; 10],
}
impl Default for Selectable {
    fn default() -> Self {
        Self { 
            is_selected: false,
            in_control_groups: [false; 10],
        }
    }
}