use super::*;

// Definition
#[derive(Component)]
pub(crate) struct Move;
#[derive(Bundle)]
pub(crate) struct BMoveB {
    
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

    let state = state.state();

    const MOVE_SWITCH_STATE_REQUIREMENTS: TreeState = ATTACK_MOVE.union(IN_AGGRO); // If attack move order and enemy is in aggro.
    if !state.contains(MOVE_SWITCH_STATE_REQUIREMENTS) {
        return;
    }

    unit_mca.0 = unit_mca.0 + 1; // Move to chase state
}

#[derive(Component)]
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

#[derive(Bundle)]
pub(crate) struct BMoveNavToMover {
    pub bang_link: BangToSwitchedMoveAsNav,
    pub move_as_nav: SwitchedMoveAsNav<BMoveNavToMover>,
    pub nav_is: NavIsReference<BMoveNavToMover>,
    pub move_is: MoveIsReference<BMoveNavToMover>,
}
ref_signature!(BMoveNavToMover);
impl Plugin for BMoveNavToMover {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            switched_reference_move_as_reference_nav_sys::<BMoveNavToMover>,
        ));
    }
}