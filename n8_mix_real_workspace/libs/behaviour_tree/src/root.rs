pub mod export;
pub mod reset;
pub mod bang;

use bevy::prelude::*;

use self::{export::signal::*, reset::reset_behaviour_sys, bang::*};

pub struct RootPlugin;
impl Plugin for RootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            propagate_spawned_root_bang_sys,
            propagate_root_bang_sys,
            
            export_when_count_sys,
            export_for_count_sys,
        ));
        app.add_systems(PostUpdate, (
            reset_behaviour_sys::<ExportWhenCount>,
            reset_behaviour_sys::<ExportForCount>,
        ));
    }
}