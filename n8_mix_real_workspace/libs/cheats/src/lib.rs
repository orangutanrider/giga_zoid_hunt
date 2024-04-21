use bevy::prelude::*;
use bevy::input::keyboard::*;
use bevy::input::*;

use mouse_pos::*;
use player_unit::public::*;
use enemy::spawn_enemy;
use rts_unit_team::*;
use rts_unit_death::*;

// Key combo to activate cheats

// Cheat button that has to be held to use cheats (like ctrl or something)

// Number input for batch spawning

pub struct CheatsPlugin;

impl Plugin for CheatsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CheatActivate>();
        app.init_resource::<CheatNum>();
        app.add_systems(Update, (
            cheat_activate_sys,
            cheat_num_sys,
            spawn_units_cheat_sys,
            spawn_enemies_cheat_sys,
            kill_enemies_sys,
            kill_player_units_sys,
        ));
    }
}

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
    first: bool,
}
impl Default for CheatNum {
    fn default() -> Self {
        Self { cheat_num: 1, first: true }
    }
}

// Each input pushes the magnitude of the total.
// So a set of inputs like this: [3, 0, 5 , 1]
// Would create this number: 3051
pub fn cheat_num_sys(
    activate: Res<CheatActivate>,
    mut res: ResMut<CheatNum>,
    input: Res<ButtonInput<KeyCode>>,
    mut keys: EventReader<KeyboardInput>,
) {
    if !activate.active { return; }

    if !input.pressed(CHEAT_BUTTON) {
        reset_cheat_num(&mut res);
        return;
    }

    for press in keys.read() {
        if !(press.state == ButtonState::Pressed) {
            continue;
        }

        let Some(num) = key_code_to_number(press.key_code) else {
            continue;
        };
        
        if res.first {
            res.cheat_num = num;
            res.first = false;
            continue;
        }

        res.cheat_num = (res.cheat_num * 10) + num;
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

fn reset_cheat_num(
    res: &mut ResMut<CheatNum>,
) {
    res.cheat_num = 1;
    res.first = true;
}

const SPAWN_UNITS_KEY: KeyCode = KeyCode::KeyP;
pub fn spawn_units_cheat_sys(
    activate: Res<CheatActivate>,
    mut res: ResMut<CheatNum>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_pos: Res<CursorWorldPos>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !activate.active { return; }

    if !input.pressed(CHEAT_BUTTON) {
        return;
    }

    if !input.just_pressed(SPAWN_UNITS_KEY) {
        return;
    }

    let location = mouse_pos.pos();

    let mut i = 0;
    while i < res.cheat_num {
        spawn_player_unit(location, &mut commands, &asset_server);
        i = i + 1;
    }

    reset_cheat_num(&mut res);
}

const SPAWN_ENEMIES_KEY: KeyCode = KeyCode::KeyO;
pub fn spawn_enemies_cheat_sys(
    activate: Res<CheatActivate>,
    mut res: ResMut<CheatNum>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_pos: Res<CursorWorldPos>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !activate.active { return; }

    if !input.pressed(CHEAT_BUTTON) {
        return;
    }

    if !input.just_pressed(SPAWN_ENEMIES_KEY) {
        return;
    }

    let location = mouse_pos.pos();

    let mut i = 0;
    while i < res.cheat_num {
        spawn_enemy(location, &mut commands, &asset_server);
        i = i + 1;
    }

    reset_cheat_num(&mut res);
}

const KILL_ALL_PLAYER_UNITS_KEY: KeyCode = KeyCode::KeyL;
pub fn kill_player_units_sys(
    activate: Res<CheatActivate>,
    input: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut DeathBang, With<PlayerTeam>>,
) {
    if !activate.active { return; }

    if !input.pressed(CHEAT_BUTTON) {
        return;
    }

    if !input.just_pressed(KILL_ALL_PLAYER_UNITS_KEY) {
        return;
    }

    for mut death in q.iter_mut() {
        death.bang();
    }
}

const KILL_ALL_ENEMIES_KEY: KeyCode = KeyCode::KeyK;
pub fn kill_enemies_sys(
    activate: Res<CheatActivate>,
    input: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut DeathBang, With<EnemyTeam>>,
) {
    if !activate.active { return; }

    if !input.pressed(CHEAT_BUTTON) {
        return;
    }

    if !input.just_pressed(KILL_ALL_ENEMIES_KEY) {
        return;
    }

    for mut death in q.iter_mut() {
        death.bang();
    }
}