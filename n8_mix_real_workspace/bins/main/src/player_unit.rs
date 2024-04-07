use std::any::TypeId;
use bevy::prelude::*;

use ref_caravan::*;
use ref_paths::*;
use ref_marks::*;

use behaviour_tree::{prelude::*, state::State as TreeState};

use rts_unit_control::prelude::*;
use rts_unit_detectors::prelude::*;

use nav_to_mover::*;

// Note:
// There are reference definitions in this that could be upgraded to be more flexible.

// ================================
// Unit Structure

#[derive(Component)]
struct Hub;
#[derive(Bundle)]
struct BHub {
    
}

#[derive(Component)]
struct Root;
#[derive(Bundle)]
struct BRoot {
    
}

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

// ================================
// Data to root state

const MOVE: TreeState = TreeState::N1;
const CHASE: TreeState = TreeState::N2;
const ATTACK: TreeState = TreeState::N3;

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

const PURE_MOVE: TreeState = TreeState::N4;
const ATTACK_MOVE: TreeState = TreeState::N5;
const ATTACK_TARGET: TreeState = TreeState::N6;
const IDLE: TreeState = TreeState::N7;

trait GenericStateBox {
    const STATE: TreeState;
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

const IN_AGGRO: TreeState = TreeState::N8;
const IN_ATTACK: TreeState = TreeState::N9.union(IN_AGGRO);

#[derive(Component)]
struct DetectionToState;

fn detection_to_state_sys(
    mut q: Query<(&mut TState, &AggroDetectorClosest, &AttackDetectorClosest, &AttackDetectorTargeted), With<DetectionToState>>,
) {
    for (mut state, aggro_close, attack_close, attack_targeted) in q.iter_mut() {
        let held: TreeState = state.state();
 
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

// ================================
// MOVE

fn move_aggro_logic_sys(
    move_q: Query<(&Bang, &ToBehaviourRoot), With<Move>>,
    mut root_q: Query<(&mut TUnitMCAMapper, &TState)>,
) {
    for (bang, to_root) in move_q.iter() {
        if !bang.is_active() {
            continue;
        }
        
        ref_caravan!(to_root::root_q((mut unit_mca, state)));

        let state = state.state();

        const CHASE_SWITCH_STATE_REQUIREMENTS: TreeState = ATTACK_MOVE.union(IN_AGGRO); // If attack move order and enemy is in aggro.
        if !state.contains(CHASE_SWITCH_STATE_REQUIREMENTS) {
            continue;
        }

        unit_mca.0 = unit_mca.0 + 1; // Move to chase state
    }
}

#[derive(Component)]
struct MoveActuator;

// The prefab systems for actuators have an oversight in how they're designed, so they don't work.
// It's cause the logic is tied to a function implementation, when it needs to be tied to a system param definition.
// Not upgrading it though, I'll just write it out manually.
fn move_actuator_sys(
    q: ActuatorQueries<MoveActuator>,
) {
    let mut node_q = q.node_q;
    let parent_q = &q.parent_q;
    
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        move_actuator(local_bang, to_parent, parent_q)
    }
}

fn move_actuator(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
) { 
    ref_caravan!(to_parent::parent_q((parent_state)););

    let actuation: TreeState = parent_state.state();
    let acutation = actuation.contains(MOVE);

    local_bang.actuator_set(acutation);
}

struct MoveRefToHubNavToRootMover;
ref_signature!(MoveRefToHubNavToRootMover);

#[derive(Bundle)]
struct BMoveNavToMover {
    pub move_as_nav: SwitchedMoveAsNav<MoveRefToHubNavToRootMover>,
    pub nav_is: NavIsReference<MoveRefToHubNavToRootMover>,
    pub move_is: MoveIsReference<MoveRefToHubNavToRootMover>,
}
pub struct MoveNavToMover;
impl Plugin for MoveNavToMover {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switched_reference_move_as_reference_nav_sys::<MoveRefToHubNavToRootMover>,
        ));
    }
}

// ================================
// Bang to switch