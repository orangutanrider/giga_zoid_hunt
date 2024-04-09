use super::*;

// Definition
#[derive(Component)]
pub(crate) struct Chase;
#[derive(Bundle)]
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
    pub hub_is_ref: HubNavIsReference
}

// Behaviour
pub(crate) fn chase_logic_sys(
    chase_q: Query<(&Bang, &ToBehaviourRoot), With<Chase>>,
    mut root_q: Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    for (bang, to_root) in chase_q.iter() {
        chase_logic(bang, to_root, &mut root_q)
    }
}

fn chase_logic(
    bang: &Bang,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    if !bang.is_active() {
        return;
    }
    
    ref_caravan!(to_root::root_q((mut unit_mca, state)));

    let state = state.state();

    const ATTACK_ORDERS: TreeState = ATTACK_MOVE.union(ATTACK_TARGET);
    if !(state.intersects(ATTACK_ORDERS) && state.contains(IN_ATTACK)) {
        return;
    }

    unit_mca.0 = unit_mca.0 + 1; // Move to attacking state
}

#[derive(Component)]
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

#[derive(Bundle)]
pub(crate) struct BChaseNavToMover {
    pub bang_link: BangToSwitchedMoveAsNav,
    pub move_as_nav: SwitchedMoveAsNav<BChaseNavToMover>,
    pub nav_is: NavIsReference<BChaseNavToMover>,
    pub move_is: MoveIsReference<BChaseNavToMover>,
}
ref_signature!(BChaseNavToMover);
impl Plugin for BChaseNavToMover {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switched_reference_move_as_reference_nav_sys::<BChaseNavToMover>,
        ));
    }
}

// Aggro to nav
// It already refers to the detector as aggro detector, so I don't think any signature is needed.
#[derive(Component)]
pub(crate) struct AggroIsReference;

#[derive(Component)]
pub(crate) struct HubNavIsReference;

#[derive(Component)]
pub(crate) struct SwitchedNavAsAggroDetectorClosest {
    pub switch: bool,
}

pub(crate) fn referenced_aggro_to_referenced_nav_sys(
    q: Query<&ToBehaviourRoot, (With<AggroIsReference>, With<HubNavIsReference>)>,
    mut root_q: Query<(&mut TNavWaypoint, &AggroDetectorClosest)>,
    target_q: Query<&GlobalTransform>,
) {
    for (to_root) in q.iter() {
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
