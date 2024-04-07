use std::any::TypeId;

use bevy::prelude::*;

use behaviour_tree::{prelude::*, state::State};
use ref_caravan::ref_caravan;
use ref_paths::*;
use rts_unit_control::commandable::{orders::{attack_move::TAttackMoveOrders, attack_target::processing::CurrentTarget, pure_move::TPureMoveOrders, TUnitOrder}, ActiveOrderTerminal};
use rts_unit_detectors::{*, distill_closest::DistillationForClosest};

#[derive(Component)]
struct AggroDetection;
#[derive(Bundle)]
struct BAggroDetection {
    
}

#[derive(Component)]
struct AttackDetection;
#[derive(Bundle)]
struct BAttackDetection {
    
}

#[derive(Component)]
struct Move;
#[derive(Bundle)]
struct BMoveB {
    
}

#[derive(Component)]
struct Chase;
#[derive(Bundle)]
struct BChase {
    
}

#[derive(Component)]
struct Attacking;
#[derive(Bundle)]
struct BAttacking {
    
}

const MOVE: State = State::N1;
const CHASE: State = State::N2;
const ATTACK: State = State::N3;

#[derive(Component)]
/// (0-2)
/// Move, Chase, Attack;
/// Mutually exclusive.
struct TUnitMCAMapper(pub u8);

fn mca_mapper_sys(
    mut q: Query<(&mut TState, &mut TUnitMCAMapper), Changed<TUnitMCAMapper>>,
) {
    for (mut state, mapper) in q.iter_mut() {
        match mapper.0 {
            0 => { // Move
                state.insert(Key::LocalComponent(TypeId::of::<TUnitMCAMapper>()), MOVE);
            },
            1 => { // Chase
                state.insert(Key::LocalComponent(TypeId::of::<TUnitMCAMapper>()), CHASE);
            },
            2 => { // Attack
                state.insert(Key::LocalComponent(TypeId::of::<TUnitMCAMapper>()), ATTACK);
            },
            _ => {
                let mut mapper = mapper;
                mapper.0 = 0;
            }
        }
    }
}

#[derive(Component)]
struct AggroDetectorClosest(pub Option<Entity>);

#[derive(Component)]
struct AttackDetectorClosest(pub Option<Entity>);

#[derive(Component)]
struct AttackDetectorTargeted(pub Option<Entity>);

fn aggro_to_tree_root_sys(
    aggro_q: Query<(&DistillationForClosest, &ToBehaviourRoot), With<AggroDetection>>,
    mut root_q: Query<&mut AggroDetectorClosest>,
) {
    for (closest, to_root) in aggro_q.iter() {
        aggro_to_tree_root(&mut root_q, closest, to_root)
    }
}
fn aggro_to_tree_root(
    root_q: &mut Query<&mut AggroDetectorClosest>,
    closest: &DistillationForClosest,
    to_root: &ToBehaviourRoot,
) {
    ref_caravan!(to_root::root_q(mut terminal));
    terminal.0 = closest.read_detection();
}

fn attack_closest_to_tree_root_sys(
    attack_q: Query<(&DistillationForClosest, &ToBehaviourRoot), With<AttackDetection>>,
    mut root_q: Query<&mut AttackDetectorClosest>,
) {
    for (closest, to_root) in attack_q.iter() {
        attack_closest_to_tree_root(&mut root_q, closest, to_root)
    }
}
fn attack_closest_to_tree_root(
    root_q: &mut Query<&mut AttackDetectorClosest>,
    closest: &DistillationForClosest,
    to_root: &ToBehaviourRoot,
) {
    ref_caravan!(to_root::root_q(mut terminal));
    terminal.0 = closest.read_detection();
}

fn attack_target_to_tree_root_sys(
    aggro_q: Query<(&DistillationForClosest, &ToBehaviourRoot), With<AttackDetection>>,
    mut root_q: Query<&mut AttackDetectorTargeted>,
) {
    for (closest, to_root) in aggro_q.iter() {
        attack_target_to_tree_root(&mut root_q, closest, to_root)
    }
}
fn attack_target_to_tree_root(
    root_q: &mut Query<&mut AttackDetectorTargeted>,
    closest: &DistillationForClosest,
    to_root: &ToBehaviourRoot,
) {
    ref_caravan!(to_root::root_q(mut terminal));
    terminal.0 = closest.read_detection();
}

const PURE_MOVE: State = State::N4;
const ATTACK_MOVE: State = State::N5;
const ATTACK_TARGET: State = State::N6;
const IDLE: State = State::N7;

trait GenericStateBox {
    const STATE: State;
}

#[derive(Component)]
/// Local transfer.
struct ControlOrdersToState;

fn control_orders_to_state_sys<OrderTerminalType: 'static, StateBox: GenericStateBox>(
    mut q: Query<(&mut TState, &ActiveOrderTerminal), With<ControlOrdersToState>>,
) {
    for (mut terminal, orders) in q.iter_mut() {
        let Some(order) = orders.current() else {
            terminal.insert(Key::LocalComponent(TypeId::of::<ControlOrdersToState>()), IDLE);
            continue;
        };

        // Validate active terminal
        if order != TypeId::of::<OrderTerminalType>() {
            continue;
        }

        terminal.insert(Key::LocalComponent(TypeId::of::<ControlOrdersToState>()), StateBox::STATE);
    }
}

const IN_AGGRO: State = State::N8;
const IN_ATTACK: State = State::N9.union(IN_AGGRO);

#[derive(Component)]
struct DetectionToState;

fn detection_to_state_sys(
    mut q: Query<(&mut TState, &AggroDetectorClosest, &AttackDetectorClosest, &AttackDetectorTargeted), With<DetectionToState>>,
) {
    for (mut state, aggro_close, attack_close, attack_targeted) in q.iter_mut() {
        let held: State = state.state();
 
        let type_id = TypeId::of::<DetectionToState>();

        if held.contains(ATTACK_TARGET){
            attack_target_detection_to_state(state, attack_targeted, type_id);
            continue;
        }

        if attack_close.0.is_some() {
            state.insert(Key::LocalComponent(type_id), IN_ATTACK);
            continue;
        }

        if aggro_close.0.is_some() {
            state.insert(Key::LocalComponent(type_id), IN_AGGRO);
            continue;
        }
    }
}

fn attack_target_detection_to_state(
    mut state: Mut<TState>,
    attack_targeted: &AttackDetectorTargeted, // if the target is in the attack range
    type_id: TypeId,
) {
    if attack_targeted.0.is_none() {
        state.insert(Key::LocalComponent(type_id), IN_AGGRO);
        return;
    }

    state.insert(Key::LocalComponent(type_id), IN_ATTACK);
}