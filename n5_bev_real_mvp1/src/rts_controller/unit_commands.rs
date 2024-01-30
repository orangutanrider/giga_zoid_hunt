mod attack;
mod pure_move;
mod stop;

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::rts_unit::control::prelude::*;
use super::mouse::RtsMouse;
use super::add_mode::AddModeInput;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            attack::InitializePlugin,
            pure_move::InitializePlugin,
            stop::InitializePlugin,
        ));
    }
}

#[derive(SystemParam)]
struct RtsCommanderContext<'w, 's> {
    unit_commander: UnitCommander<'w, 's>,
    mouse: RtsMouse<'w>,
    add_mode_input: AddModeInput<'w>,
}