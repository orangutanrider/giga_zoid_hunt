use super::*;

// Definition
#[derive(Component)]
pub(crate) struct Chase;
#[derive(Bundle)]
pub(crate) struct BChase {
    
}

// Behaviour
pub(crate) fn chase_attack_logic_sys(
    chase_q: Query<(&Bang, &ToBehaviourRoot), With<Chase>>,
    mut root_q: Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    for (bang, to_root) in chase_q.iter() {
        chase_attack_logic(bang, to_root, &mut root_q)
    }
}

fn chase_attack_logic(
    bang: &Bang,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TUnitIMCAMapper, &TState)>,
) {
    if !bang.is_active() {
        return;
    }
    
    ref_caravan!(to_root::root_q((mut unit_mca, state)));

    let state = state.state();

    const CHASE_SWITCH_STATE_REQUIREMENTS: TreeState = ATTACK_MOVE.union(IN_ATTACK);
    if !state.contains(CHASE_SWITCH_STATE_REQUIREMENTS) {
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
#[derive(Component)]
pub(crate) struct AggroIsReference<S: RefSignature> {
    signature: PhantomData<S>,
}

#[derive(Component)]
pub(crate) struct SwitchedNavAsAggroDetectorClosest<S: RefSignature> {
    pub switch: bool,
    signature: PhantomData<S>,
}

pub(crate) fn referenced_aggro_to_referenced_nav_sys<S: RefSignature>(
    q: Query<&ToBehaviourRoot, (With<AggroIsReference<S>>, With<NavIsReference<S>>)>,
    mut root_q: Query<(&mut TNavWaypoint, &AggroDetectorClosest)>,
    target_q: Query<&GlobalTransform>,
) {
    for (to_root) in q.iter() {
        referenced_aggro_to_referenced_nav::<S>(to_root, &mut root_q, &target_q);
    }
} 

fn referenced_aggro_to_referenced_nav<S: RefSignature>( 
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
