mod selection_box;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use super::mouse::RtsMouse;
use super::add_mode::AddModeInput;
use crate::rts_unit::control::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(selection_box::InitializePlugin);
    }
}

#[derive(SystemParam)]
struct RtsSelectorContext<'w, 's> {
    unit_selector: UnitSelector<'w, 's>,
    mouse: RtsMouse<'w>,
    add_mode_input: AddModeInput<'w>,
}