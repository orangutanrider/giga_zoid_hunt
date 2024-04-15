use bang_colour::BangColour;

use super::*;

// Definition
#[derive(Component, Default)]
pub(crate) struct Idle;
#[derive(Bundle, Default)]
pub(crate) struct BIdle {
    pub flag: Idle,

    pub to_root: ToBehaviourRoot,
    pub to_parent: ToParentNode,
    pub bang: Bang,
    pub propagator: ActuatorPropagator,
    pub actuator: IdleActuator,

    pub to_control: ToControl,
    pub to_nav: ToNav,
    pub to_mover: ToMover,

    pub no_move: RefdMoverIsZeroWhenBang,
    
    pub bang_colour: BangColour,
}

// Behaviour
pub(crate) fn idle_logic_sys(
    chase_q: Query<(&Bang, &ToBehaviourRoot), With<Idle>>,
    mut root_q: Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    for (bang, to_root) in chase_q.iter() {
        idle_logic(bang, to_root, &mut root_q)
    }
}

fn idle_logic(
    bang: &Bang,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    if !bang.is_active() {
        return;
    }
    
    ref_caravan!(to_root::root_q((mut unit_mca, state)));

    const MOVE_ORDERS: TreeState = PURE_MOVE.union(ATTACK_MOVE);

    let state: TreeState = state.state();
    if state.intersects(MOVE_ORDERS) {
        unit_mca.0 = 1; // Move to move state
        return;
    }
    else if state.contains(ATTACK_TARGET) {
        unit_mca.0 = 2; // Move to chase state
    }
    else if state.contains(IN_ATTACK) { // Hold position is default
        unit_mca.0 = 3; // Move to attacking state
        return;
    }
}

#[derive(Component, Default)]
pub(crate) struct IdleActuator;

pub(crate) fn idle_actuator_sys(
    q: ActuatorQueries<IdleActuator>,
) {
    let mut node_q = q.node_q;
    let parent_q = &q.parent_q;
    
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        idle_actuator(local_bang, to_parent, parent_q)
    }
}

fn idle_actuator(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
) { 
    ref_caravan!(to_parent::parent_q((parent_state)););

    let actuation: TreeState = parent_state.state();
    let actuation = actuation.contains(IDLE);

    local_bang.actuator_set(actuation);
}