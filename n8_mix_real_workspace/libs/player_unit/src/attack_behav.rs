use super::*;

#[derive(Component)]
pub(crate) struct AttackActuator;

pub(crate) fn attack_actuator_sys(
    q: ActuatorQueries<AttackActuator>,
) {
    let mut node_q = q.node_q;
    let parent_q = &q.parent_q;
    
    for (local_bang, propagator, to_parent) in node_q.iter_mut() {
        if !propagator.is_propagating() {
            continue;
        }
        attack_actuator(local_bang, to_parent, parent_q)
    }
}

fn attack_actuator(
    mut local_bang: Mut<Bang>,
    to_parent: &ToParentNode,
    parent_q: &Query<&TState>,
) { 
    ref_caravan!(to_parent::parent_q((parent_state)););

    let actuation: TreeState = parent_state.state();
    let actuation = actuation.contains(ATTACK);

    local_bang.actuator_set(actuation);
}