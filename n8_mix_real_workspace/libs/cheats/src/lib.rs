use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::*;

// Key combo to activate cheats

// Cheat button that has to be held to use cheats (like ctrl or something)

// Number input for batch spawning

#[derive(Resource)]
pub struct CheatActivate{
    active: bool,
    combo_index: usize,
} 
impl Default for CheatActivate {
    fn default() -> Self {
        return Self{
            active: false,
            combo_index: 0,
        }
    }
}

// code:
// CHEATS
pub const CHEAT_COMBO: [KeyCode; 6] = [KeyCode::KeyC, KeyCode::KeyH, KeyCode::KeyE, KeyCode::KeyA, KeyCode::KeyT, KeyCode::KeyS];
pub const SUBMISSION_KEY: KeyCode = KeyCode::Enter;

pub fn cheat_activate_sys(
    mut res: ResMut<CheatActivate>,
    input: Res<ButtonInput<KeyCode>>,
    mut keys: EventReader<KeyboardInput>,
) {
    if res.active { return; }

    let index = res.combo_index;

    // If the index has reached the end of the combo and the submission key has been pressed.
    // Then activate cheats.
    if input.just_pressed(SUBMISSION_KEY) && index == (CHEAT_COMBO.len() - 1) {
        res.active = true;
        return;
    }

    let current = CHEAT_COMBO[index];

    // Read key presses
    // If the key press matches the current key in the combo at the index.
    // Then advance the index.
    // But if it doesn't match, reset the index to 0
    for press in keys.read() {
        if !(press.state == ButtonState::Pressed) {
            continue;
        }

        if press.key_code == current { // Continue
            res.combo_index = res.combo_index + 1;
            return;
        }
        else { // Reset
            res.combo_index = 0;
            return;
        }
    }
}

pub const CHEAT_BUTTON: KeyCode = KeyCode::Semicolon;

#[derive(Resource)]
pub struct CheatNum{
    cheat_num: u32,
    input_power: u32,
}

pub fn cheat_num_sys(
    mut res: ResMut<CheatNum>,
    input: Res<ButtonInput<KeyCode>>,
    mut keys: EventReader<KeyboardInput>,
) {
    if !input.pressed(CHEAT_BUTTON) {
        res.cheat_num = 1;
        return;
    }

    for press in keys.read() {
        if !(press.state == ButtonState::Pressed) {
            continue;
        }

        let num = key_code_to_number(press.key_code);
    }
}

fn key_code_to_number(key: KeyCode) -> Option<u32> {
    match key {
        KeyCode::Digit0 => Some(0),
        KeyCode::Digit1 => Some(1),
        KeyCode::Digit2 => Some(2),
        KeyCode::Digit3 => Some(3),
        KeyCode::Digit4 => Some(4),
        KeyCode::Digit5 => Some(5),
        KeyCode::Digit6 => Some(6),
        KeyCode::Digit7 => Some(7),
        KeyCode::Digit8 => Some(8),
        KeyCode::Digit9 => Some(9),
        _ => None,
    }
}