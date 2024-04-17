/* 
use behaviour_tree::bang::Bang;
use behaviour_tree::state::terminal::TState;
use bevy::prelude::*;
use enemy::chase::Chase;
use player_unit::test_enemy::spawn_test_enemy;
use player_unit::Root;
use rts_unit_control::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_enemy_dbg_sys);

        app.add_systems(Update, (
            //prnt_units_sys,
            //prnt_if_targeted_by_exists,
            prnt_targeted_by,
            prnt_attack_target_data_sys,
            prnt_state_sys,
            prnt_order_type_sys,
            prnt_hub_sys,
            prnt_movers_sys,
            prnt_nav_sys,
            prnt_enemy_chase_head_position,
            prnt_order_data_sys::<TPureMoveOrders, PureMoveOrder, PureMove>,
            prnt_order_data_sys::<TAttackMoveOrders, AttackMoveOrder, AMove>,
            prnt_order_data_sys::<TAttackTargetOrders, AttackTargetOrder, Targeted>,
        ));
    }
}

fn spawn_test_enemy_dbg_sys(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    spawn_test_enemy(Vec2::new(0.0, 64.0), &mut commands, &asset_server);
}

fn prnt_units_sys(
    q: Query<&Root>,
) {
    let mut e = 0;
    for unit in q.iter() {
        e = e + 1;
    }

    println!("\n {}", e);
}

fn prnt_order_type_sys(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&TActiveOrderType>
) {
    if !input.just_pressed(KeyCode::KeyK) {
        return;
    }
    println!("");

    for (type_terminal) in q.iter() {
        let current = type_terminal.current();

        println!("{:?}", current);
    }
}

trait KeyCodeBox {
    const KEY_CODE: KeyCode;
}
struct AMove;
impl KeyCodeBox for AMove {
    const KEY_CODE: KeyCode = KeyCode::KeyI;
}
struct Targeted;
impl KeyCodeBox for Targeted {
    const KEY_CODE: KeyCode = KeyCode::KeyO;
}
struct PureMove;
impl KeyCodeBox for PureMove {
    const KEY_CODE: KeyCode = KeyCode::KeyP;
}

fn prnt_order_data_sys<T, O, K>(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&T>
) where
    T: TUnitOrder<O> + Component,
    O: 'static + Copy + std::fmt::Debug,
    K: KeyCodeBox,
{
    if !input.just_pressed(K::KEY_CODE) {
        return;
    }
    println!("");

    for type_terminal in q.iter() {
        let current = type_terminal.current();

        println!("{:?}", current);
    }
}

fn prnt_state_sys(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&TState>,
) {
    if !input.just_pressed(KeyCode::KeyL) {
        return;
    }
    println!("");

    for state in q.iter() {
        let state: behaviour_tree::prelude::State = state.state();
        println!("{:?}", state);
        prnt_state_mapping(state);
    }
}

use player_unit::state_to_root::*;
fn prnt_state_mapping(
    state: behaviour_tree::prelude::State
) {
    println!("State mappings: ");
    print!(" | MOVE: {}", state.contains(MOVE));
    print!(" | CHASE: {}", state.contains(CHASE));
    print!(" | ATTACK: {}", state.contains(ATTACK));
    print!(" | IDLE: {}", state.contains(IDLE));
    print!(" | PURE_MOVE: {}", state.contains(PURE_MOVE));
    print!(" | ATTACK_MOVE: {}", state.contains(ATTACK_MOVE));
    print!(" | ATTACK_TARGET: {}", state.contains(ATTACK_TARGET));
    print!(" | IN_AGGRO: {}", state.contains(IN_AGGRO));
    print!(" | IN_ATTACK: {}", state.contains(IN_ATTACK_RANGE));
    println!("");
}

use player_unit::Hub;
use rts_unit_detectors::distill_closest::DistillationForClosest;
use rts_unit_detectors::distill_target::DistillationForTarget;
use rts_unit_detectors::TIntersectionsAggregate;
use rts_unit_movers::TMoveVector;
use rts_unit_nav::NavVectorOutput;
use rts_unit_nav::TNavWaypoint;
fn prnt_hub_sys(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&Bang, With<Hub>>,
) {
    if !input.just_pressed(KeyCode::KeyM) {
        return;
    }
    println!("");

    for bang in q.iter() {
        println!("Hub Bang: {}", bang.is_active());
    }
}

fn prnt_movers_sys(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&TMoveVector>,
) {
    if !input.just_pressed(KeyCode::KeyN) {
        return;
    }
    println!("");

    for mover in q.iter() {
        println!("Mover: {}", mover.0);
    }
}

fn prnt_nav_sys(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<(Option<&TNavWaypoint>, Option<&NavVectorOutput>)>,
) {
    if !input.just_pressed(KeyCode::KeyB) {
        return;
    }
    println!("");

    for (nav, output) in q.iter() {

        if let Some(nav) = nav {
            println!("Nav: {}", nav.0);
        }

        if let Some(output) = output {
            println!("Nav Output: {}", output.0);
        }
    }
}

fn prnt_attack_target_data_sys(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<(Option<&TAttackTargetOrders>, Option<&TCurrentTarget>)>,
) {
    if !input.just_pressed(KeyCode::KeyH) {
        return;
    }
    println!("");

    for (orders, current) in q.iter() {

        if let Some(orders) = orders {
            let mut index = 0;
            for order in orders.iter() {
                println!("Attack Target Order {}: {:?}", index, order.target);
                index = index + 1;
            }
        }

        if let Some(current) = current {
            println!("Current: {:?}", current.read());
        }
    }
}

fn prnt_if_targeted_by_exists(
    q: Query<&TargetedBy>,
) {
    let mut index = 0;
    for _ in q.iter() {
        index = index + 1;
    }
    if index > 0 {
        println!("{} targeted by components", index);
    }
}

fn prnt_targeted_by(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&TargetedBy>,
) {
    if !input.just_pressed(KeyCode::KeyY) {
        return;
    }
    println!("");

    for targeted_by in q.iter() {
        println!("TargetedBy: {}", targeted_by.read().len());
    }
}

/* 
fn prnt_detection_data(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<(&TIntersectionsAggregate, &DistillationForClosest)>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }
    println!("");

    for targeted_by in q.iter() {
        println!("TargetedBy: {}", targeted_by.read().len());
    }
}
*/

fn prnt_enemy_chase_head_position(
    input: Res<ButtonInput<KeyCode>>,
    q: Query<&GlobalTransform, With<Chase>>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }
    println!("");

    for transform in q.iter() {
        let transform = transform.translation();
        println!("Chase: {}", transform);
    }
}
*/