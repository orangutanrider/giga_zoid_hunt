use bevy::{ecs::system::SystemParam, prelude::*};

use rts_unit_control::prelude::*;

#[derive(SystemParam)]
pub struct StopInput<'w> {
    keys: Res<'w, ButtonInput<KeyCode>>,
}
impl<'w> StopInput<'w> {
    const KEYS: [KeyCode; 1] = [KeyCode::KeyS];

    fn just_pressed(&self) -> bool {
        return self.keys.any_just_pressed(Self::KEYS);
    }
}

pub fn command_stop_sys(
    input: StopInput, 
    mut q: Query<&mut ClearOrdersBang, With<Selected>>
) {
    if !input.just_pressed() {
        return;
    }

    for mut bang in q.iter_mut() {
        bang.bang();
    }
}