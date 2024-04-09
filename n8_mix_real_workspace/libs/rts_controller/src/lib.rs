pub mod command;
pub mod selection;
pub mod rapier;
pub mod add_mode;

use command::{attack::command_attack_sys, pure_move::command_pure_move_sys, stop::command_stop_sys};
use bevy::prelude::*;
use selection::r#box::{box_origin_sys, box_selection_sys, BoxOrigin};


pub struct RTSControllerPlugin;

impl Plugin for RTSControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoxOrigin>();

        app.add_systems(Update, (
            box_origin_sys,
            box_selection_sys,
            command_stop_sys,
            command_pure_move_sys,
            command_attack_sys,
        ));
    }
}