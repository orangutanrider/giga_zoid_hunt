pub mod export;
pub mod reset;
pub mod bang;

use bevy::prelude::*;

use self::{export::signal::*, reset::reset_behaviour_sys};

pub struct RootPlugin;
impl Plugin for RootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            export_when_count_sys,
            export_for_count_sys,
            reset_behaviour_sys::<ExportWhenCount>,
            reset_behaviour_sys::<ExportForCount>,
        ));
    }
}