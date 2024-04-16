use bang_colour::BangColour;
use bevy::prelude::*;
use rts_unit_movers::ToMover;
use rts_unit_team::PlayerTeam;

use crate::{AggroDetectorClosest, ATTACK_TARGET, CHASE, IN_ATTACK, PURE_MOVE};

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

use rts_unit_control::{commandable::OrderProcessedAgar, *};
use rts_unit_nav::*;

// Definition
#[derive(Component, Default)]
pub(crate) struct Chase;
#[derive(Bundle, Default)]
pub(crate) struct BChase {
    pub flag: Chase,

    pub to_root: ToBehaviourRoot,
    pub to_parent: ToParentNode,
    pub bang: Bang,
    pub propagator: ActuatorPropagator,
    pub actuator: ChaseActuator,

    pub nav_to_mover: BChaseNavToMover,

    // aggro to nav
    pub aggro_is_ref: AggroIsReference,
    pub nav_as_aggro: SwitchedNavAsAggroDetectorClosest,
    pub hub_is_ref: HubNavIsReference,
    
    // attack target to nav
    pub attack_target_to_nav: BChaseControlToNav,

    pub to_control: ToControl,
    pub to_nav: ToNav,
    pub to_mover: ToMover,

    pub bang_colour: BangColour,

    pub team_affiliation: PlayerTeam,
}

// Behaviour
pub(crate) fn chase_logic_sys(
    chase_q: Query<(&Bang, &ToBehaviourRoot), With<Chase>>,
    mut root_q: Query<(&mut TUnitIMCAMapper, &TState, &OrderProcessedAgar)>,
) {
    for (bang, to_root) in chase_q.iter() {
        chase_logic(bang, to_root, &mut root_q)
    }
}

fn chase_logic(
    bang: &Bang,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TUnitIMCAMapper, &TState, &OrderProcessedAgar)>,
) {
    if !bang.is_active() {
        return;
    }
    
    ref_caravan!(to_root::root_q((mut unit_mca, state, agar)));

    if agar.is_active() {
        unit_mca.0 = 0; // Move to idle
        return;
    }

    let state: TreeState = state.state();

    if state.contains(PURE_MOVE) {
        unit_mca.0 = 1; // move to move
        return;
    }

    const ATTACK_ORDERS: TreeState = ATTACK_MOVE.union(ATTACK_TARGET);
    if !(state.intersects(ATTACK_ORDERS) && state.contains(IN_ATTACK)) {
        return;
    }

    unit_mca.0 = unit_mca.0 + 1; // Move to attacking state
}

#[derive(Component, Default)]
pub(crate) struct ChaseActuator;

pub(crate) fn chase_actuator_sys(
    q: ActuatorQueries<ChaseActuator>,
) {
    let mut node_q = q.node_q;
    let parent_q = &q.parent_q;
    
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        chase_actuator(local_bang, to_parent, parent_q)
    }
}

fn chase_actuator(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
) { 
    ref_caravan!(to_parent::parent_q((parent_state)););

    let actuation: TreeState = parent_state.state();
    let actuation = actuation.contains(CHASE);

    local_bang.actuator_set(actuation);
}

#[derive(Bundle, Default)]
pub(crate) struct BChaseNavToMover {
    pub bang_link: BangToSwitch<BChaseNavToMover>,
    pub move_as_nav: SwitchedMoveAsNav<BChaseNavToMover>,
    pub nav_is: NavIsReferenceMover<BChaseNavToMover>,
    pub move_is: MoveIsReference<BChaseNavToMover>,
}
ref_signature!(BChaseNavToMover);
pub struct BChaseNavToMoverPlugin;
impl Plugin for BChaseNavToMoverPlugin {
    fn build(&self, app: &mut App) {
        type NavAsMove = SwitchedMoveAsNav<BChaseNavToMover>;
        type BangLink = BangToSwitch<BChaseNavToMover>;

        app.add_systems(Update, (
            bang_to_switch_sys::<NavAsMove, BangLink, BChaseNavToMover>,
            switched_reference_move_as_reference_nav_sys::<BChaseNavToMover>,
        ));
    }
}

// Aggro to nav
// It already refers to the detector as aggro detector, so I don't think any signature is needed.
#[derive(Component, Default)]
pub(crate) struct AggroIsReference;

#[derive(Component, Default)]
pub(crate) struct HubNavIsReference;

#[derive(Component)]
pub(crate) struct SwitchedNavAsAggroDetectorClosest {
    pub switch: bool,
}
impl Default for SwitchedNavAsAggroDetectorClosest {
    fn default() -> Self {
        Self { switch: false }
    }
}

pub fn bang_to_switched_aggro_to_nav(
    mut q: Query<(&Bang, &mut SwitchedNavAsAggroDetectorClosest)>
) {
    for (bang, mut switch) in q.iter_mut(){
        switch.switch = bang.is_active();
    }
}

pub(crate) fn referenced_aggro_to_referenced_nav_sys(
    q: Query<(&ToBehaviourRoot, &SwitchedNavAsAggroDetectorClosest), (With<AggroIsReference>, With<HubNavIsReference>)>,
    mut root_q: Query<(&mut TNavWaypoint, &AggroDetectorClosest)>,
    target_q: Query<&GlobalTransform>,
) {
    for (to_root, switch) in q.iter() {
        if !switch.switch {
            continue;
        }
        referenced_aggro_to_referenced_nav(to_root, &mut root_q, &target_q);
    }
} 

fn referenced_aggro_to_referenced_nav( 
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TNavWaypoint, &AggroDetectorClosest)>,
    target_q: &Query<&GlobalTransform>,
) {
    ref_caravan!(to_root::root_q((mut nav, aggro)));

    let Some(target) = aggro.0 else {
        return; 
    };
    let Ok(transform) = target_q.get(target) else {
        return;
    };
    let waypoint = transform.translation().truncate();
    nav.0 = waypoint;
}


#[derive(Bundle, Default)]
pub(crate) struct BChaseControlToNav {
    pub bang_link: BangToSwitch<BChaseControlToNav>,
    pub nav_is_ref: NavIsReferenceControl<BChaseControlToNav>,
    pub control_is_ref: ControlIsReference<BChaseControlToNav>,
    pub as_attack_target: SwitchedNavAsAttackTarget<BChaseControlToNav>
}
ref_signature!(BChaseControlToNav);
pub struct BChaseControlToNavPlugin;
impl Plugin for BChaseControlToNavPlugin {
    fn build(&self, app: &mut App) {
        type AttackTargetAsNav = SwitchedNavAsAttackTarget<BChaseControlToNav>;
        type BangLink = BangToSwitch<BChaseControlToNav>;

        app.add_systems(Update, (
            bang_to_switch_sys::<AttackTargetAsNav, BangLink, BChaseControlToNav>,
            reference_attack_target_as_reference_nav_sys::<BChaseControlToNav>,
        ));
    }
}