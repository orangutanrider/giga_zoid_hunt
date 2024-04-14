use bevy::prelude::*;
use rts_unit_movers::ToMover;

use crate::{ATTACK_TARGET, PURE_MOVE};

use super::{
    TUnitIMCAMapper,
    state_to_root::{
        ATTACK_MOVE,
        IN_AGGRO,
        MOVE
    },
    common::*,
};

pub(crate) use behaviour_tree::{prelude::*, state::State as TreeState};
use ref_caravan::*;
use ref_paths::*;
use ref_marks::*;

use nav_to_mover::{
    NavIsReference as NavIsReferenceMover,
    *
};

use control_to_nav::{
    NavIsReference as NavIsReferenceControl,
    *
};

use rts_unit_control::*;
use rts_unit_nav::*;

// Definition
#[derive(Component, Default)]
pub(crate) struct Move;
#[derive(Bundle, Default)]
pub(crate) struct BMoveB {
    pub flag: Move,

    pub to_root: ToBehaviourRoot,
    pub to_parent: ToParentNode,
    pub bang: Bang,
    pub propagator: ActuatorPropagator,
    pub actuator: MoveActuator,

    pub nav_to_mover: BMoveNavToMover,
    pub control_to_nav: BMoveControlToNav,

    pub to_control: ToControl,
    pub to_nav: ToNav,
    pub to_mover: ToMover,
}

// Behaviour
pub(crate) fn move_aggro_logic_sys(
    move_q: Query<(&Bang, &ToBehaviourRoot), With<Move>>,
    mut root_q: Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    for (bang, to_root) in move_q.iter() {
        move_aggro_logic(bang, to_root, &mut root_q);
    }
}

pub(crate) fn move_aggro_logic(
    bang: &Bang,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    if !bang.is_active() {
        return;
    }
    
    ref_caravan!(to_root::root_q((mut unit_mca, state)));

    let state: TreeState = state.state();

    if state.contains(ATTACK_TARGET) {
        unit_mca.0 = 2; // move to chase 
        return;
    }

    const MOVE_SWITCH_STATE_REQUIREMENTS: TreeState = ATTACK_MOVE.union(IN_AGGRO); // If attack move order and enemy is in aggro.
    if state.contains(MOVE_SWITCH_STATE_REQUIREMENTS) {
        unit_mca.0 = unit_mca.0 + 1; // Move to chase state
        return;
    }

    if !(state.intersects(ATTACK_MOVE.union(PURE_MOVE))) {
        unit_mca.0 = 0;
    }

}

#[derive(Component, Default)]
pub(crate) struct MoveActuator;

// The prefab systems for actuators have an oversight in how they're designed, so they don't work.
// It's cause the logic is tied to a function implementation, when it needs to be tied to a system param definition.
// Not upgrading it though, I'll just write it out manually.
pub(crate) fn move_actuator_sys(
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

#[derive(Bundle, Default)]
pub(crate) struct BMoveNavToMover {
    pub bang_link: BangToSwitch<BMoveNavToMover>,
    pub move_as_nav: SwitchedMoveAsNav<BMoveNavToMover>,
    pub nav_is: NavIsReferenceMover<BMoveNavToMover>,
    pub move_is: MoveIsReference<BMoveNavToMover>,
}
ref_signature!(BMoveNavToMover);
pub struct BMoveNavToMoverPlugin;
impl Plugin for BMoveNavToMoverPlugin {
    fn build(&self, app: &mut App) {
        type NavAsMove = SwitchedMoveAsNav<BMoveNavToMover>;
        type BangLink = BangToSwitch<BMoveNavToMover>;

        app.add_systems(Update, (
            bang_to_switch_sys::<NavAsMove, BangLink, BMoveNavToMover>,
            switched_reference_move_as_reference_nav_sys::<BMoveNavToMover>,
        ));
    }
}

#[derive(Bundle, Default)]
pub(crate) struct BMoveControlToNav {
    pub bang_link: BangToSwitch<BMoveControlToNav>,
    pub nav_is_ref: NavIsReferenceControl<BMoveControlToNav>,
    pub control_is_ref: ControlIsReference<BMoveControlToNav>,
    pub as_attack_move: SwitchedNavAsAttackMove<BMoveControlToNav>,
    pub as_pure_move: SwitchedNavAsPureMove<BMoveControlToNav>,
}
ref_signature!(BMoveControlToNav);
pub struct BMoveControlToNavPlugin;
impl Plugin for BMoveControlToNavPlugin {
    fn build(&self, app: &mut App) {
        type PureMoveAsNav = SwitchedNavAsPureMove<BMoveControlToNav>;
        type AttackMoveAsNav = SwitchedNavAsAttackMove<BMoveControlToNav>;
        type BangLink = BangToSwitch<BMoveControlToNav>;

        app.add_systems(Update, (
            bang_to_switch_sys::<PureMoveAsNav, BangLink, BMoveControlToNav>,
            bang_to_switch_sys::<AttackMoveAsNav, BangLink, BMoveControlToNav>,
            reference_attack_move_as_reference_nav_sys::<BMoveControlToNav>,
            reference_pure_move_as_reference_nav_sys::<BMoveControlToNav>
        ));
    }
}

/*
    The data transmission libraries have a fault.
    Example:
	    from x to y
	    from x to z
    If both have definitions for x as a reference (XIsReference), then you can see the issue here; Two components that are the same thing, but in different types.

    So those reference flags, they need to be in seperate libs, not related to transmission.
    Then the transmission libs, only contain flags for data transformation.

    A ting, for the future. 
*/