use bevy::prelude::*;
use super::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing unit_selector");
        app.add_systems(Update, select_unit_via_alpha_numeric);
    }
}

fn select_unit_via_alpha_numeric(    
    mut key_evr: EventReader<KeyboardInput>,
    mut unit_query: Query<&mut TestUnit, With<TestUnit>>
){
    let mut numeric_key: u32 = 0;
    for ev in key_evr.iter() {
        if !ev.state.is_pressed(){
            continue;
        }
        numeric_key = ev.scan_code;
    }
    if numeric_key < 2 || numeric_key > 11 {
        return;
    }

    let mut index = 0;
    for mut test_unit in unit_query.iter_mut(){
        index+=1;

        if numeric_key - 1 == index {
            test_unit.selected = !test_unit.selected;
        }
    }
}

// https://bevy-cheatbook.github.io/input/keyboard.html
fn print_keycode(
    mut key_evr: EventReader<KeyboardInput>,
){
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {}
        }
    }
}