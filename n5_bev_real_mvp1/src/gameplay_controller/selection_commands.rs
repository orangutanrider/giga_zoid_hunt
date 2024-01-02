use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use mouse_tracking::MousePosWorld;

use crate::unit::commandable::*;
use crate::unit::orders::AttackMoveOrder;
use crate::unit::orders::PureMovementOrder;
use super::selection::*;
use super::rapier_mouse::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing gameplay_controller::selection_commands");
        app
        .add_systems(Update, (
            stop_input,
            attack_input,
            move_input,
        ));
    }
}

/// Update
fn stop_input(
    keys_input: Res<Input<KeyCode>>,
    selection: ResMut<SelectionContext>,
    mut commandable_q: Query<&mut Commandable>,
) {
    if !keys_input.just_pressed(KeyCode::S) {
        return;
    } // If S pressed

    for entity in selection.selected.iter() {
        let commandable = commandable_q.get_mut(*entity);
        let mut commandable = commandable.unwrap();
        commandable.clear_orders();
    }
}

/// Update
fn attack_input(
    keys_input: Res<Input<KeyCode>>,
    selection: ResMut<SelectionContext>,
    mut commandable_q: Query<&mut Commandable>,
    mouse_world: Res<MousePosWorld>,
    rapier: Res<RapierContext>,
) {
    if !keys_input.just_pressed(KeyCode::A){
        return;
    } // If A pressed
    println!("Giving attack move command to selection");

    let shift_held = keys_input.pressed(KeyCode::ShiftLeft);
    if shift_held {
        println!("Shift has been held for attack move command");
    }

    let waypoint = mouse_world.truncate();
    let cast_result = single_cast(rapier, waypoint);

    // If single cast has hit enemy
    // Do attack target
    // todo!();

    // Otherwise do attack move

    println!("Num selected for the attack move command: {}", selection.selected.len());
    for entity in selection.selected.iter() {
        let commandable = commandable_q.get_mut(*entity);
        let mut commandable = commandable.unwrap();

        if shift_held == false {
            commandable.clear_orders();
        }

        commandable.give_attack_move_order(
            // There'll have to be somekind of waypoint organising system in the future, so that units don't all move onto the same spot
            AttackMoveOrder{waypoint}
        )
    }
}

/// Update
fn move_input(
    keys_input: Res<Input<KeyCode>>,
    selection: ResMut<SelectionContext>,
    mut commandable_q: Query<&mut Commandable>,
    mouse_world: Res<MousePosWorld>,
) {
    if !keys_input.just_pressed(KeyCode::D){
        return;
    } // If D pressed

    let shift_held = keys_input.pressed(KeyCode::ShiftLeft);

    let waypoint = mouse_world.truncate();

    for entity in selection.selected.iter() {
        let commandable = commandable_q.get_mut(*entity);
        let mut commandable = commandable.unwrap();

        if shift_held == false {
            commandable.clear_orders();
        }

        commandable.give_pure_move_order(
            // There'll have to be somekind of waypoint organising system in the future, so that units don't all move onto the same spot
            PureMovementOrder{waypoint}
        )
    }
}