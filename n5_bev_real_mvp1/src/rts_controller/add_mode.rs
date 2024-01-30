use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct AddModeInput<'w> {
    keys: Res<'w, Input<KeyCode>>,
}
impl<'w> AddModeInput<'w> {
    const KEYS: [KeyCode; 2] = [KeyCode::ShiftLeft, KeyCode::ShiftRight];

    pub fn is_pressed(&self) -> bool {
        return self.keys.any_pressed(Self::KEYS);
    }
}