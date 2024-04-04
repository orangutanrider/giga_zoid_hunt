use bevy::{ecs::system::SystemParam, prelude::*};
use mouse_pos::CursorWorldPos;

use rts_unit_control::prelude::*;

use crate::add_mode::AddModeInput;

#[derive(SystemParam)]
pub struct PureMoveInput<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    add_mode: AddModeInput<'w>,
    mouse_pos: Res<'w, CursorWorldPos>,
}
impl<'w> PureMoveInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::KeyD];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }

    pub fn add_mode(&self) -> bool {
        return self.add_mode.is_pressed()
    }

    pub fn pos(&self) -> Vec2 {
        return self.mouse_pos.pos();
    }
}

pub fn command_pure_move_sys(
    input: PureMoveInput, 
    mut commands: SelectionCommands<TPureMoveOrders, PureMoveOrder>,
) {
    if !input.just_pressed() {
        return;
    }

    let order = PureMoveOrder::new(input.pos());
    commands.command(input.add_mode(), &order);
}