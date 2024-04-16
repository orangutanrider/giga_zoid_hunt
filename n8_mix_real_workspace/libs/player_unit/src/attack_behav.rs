use super::*;
use attack_laser::LaserVisualsOnAttack;
use bang_colour::BangColour;
use rts_direct_attack::*;

// Hold target locally.
// Update target, while bang is inactive.
// If attack move, target is closest in range from root.
// If attack target, target is AttackDetectorTargeted.

// Once active, (nothing should be updating those targets).
// Once active, constant (0,0) to mover.
// Once active, start timer.
// At timer point x, bang the direct attack terminal.
// At timer end, set march to 1. (If target got killed, then the progression of the orders will automatically bring it to an idle state.).

// It could be that there is another state, ChaseTarget, that this switches to instead of switching to move.
// The benefit of that would be that, if the unit doesn't kill their target, then they would still focus that target, and not re-assign their target when entering back into chase state.
// This is good enough though, I don't know if that would be better either, I don't think it works like that in StarCraft.

// Definition
#[derive(Component, Default)]
pub(crate) struct Attack;
#[derive(Bundle, Default)]
pub(crate) struct BAttack {
    pub flag: Attack,

    pub to_root: ToBehaviourRoot,
    pub to_parent: ToParentNode,
    pub bang: Bang,
    pub propagator: ActuatorPropagator,
    pub actuator: AttackActuator,

    pub targeted: AttackTarget,
    pub timer: AttackTimer,
    pub trigger: AttackTrigger,
    pub end: AttackEndTrigger,

    pub attack: DirectAttackBang,
    pub damage: DirectAttackPower,

    pub to_control: ToControl,
    pub to_nav: ToNav,
    pub to_mover: ToMover,

    pub no_move: RefdMoverIsZeroWhenBang,

    pub bang_colour: BangColour,
    pub attack_laser: LaserVisualsOnAttack,

    pub team_affiliation: PlayerTeam,
}

pub fn attack_behav_sys(
    chase_q: Query<(&Bang, &ToBehaviourRoot), With<Attack>>,
    mut root_q: Query<(&mut TUnitIMCAMapper, &TState, &OrderProcessedAgar)>,
) {
    for (bang, to_root) in chase_q.iter() {
        attack_logic(bang, to_root, &mut root_q)
    }
}

// While attacking.
// Any update to the orders, should interupt the attack.
// Also, it should exit from the attack order, when other orders are active.
fn attack_logic(
    bang: &Bang,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<(&mut TUnitIMCAMapper, &TState, &OrderProcessedAgar)>,
) {
    if !bang.is_active() {
        return;
    }
    
    ref_caravan!(to_root::root_q((mut unit_mca, state, agar)));

    let state: TreeState = state.state();

    if state.contains(PURE_MOVE) {
        unit_mca.0 = 1; // Move to move
        return
    }

    if agar.is_active() {
        unit_mca.0 = 0; // Move to idle
        return;
    }
}

// Target handling
#[derive(Component)]
pub struct AttackTarget(Option<Entity>);
impl Default for AttackTarget {
    fn default() -> Self {
        Self(None)
    }
}

pub fn target_update_sys(
    mut q: Query<(&mut AttackTarget, &Bang, &ToBehaviourRoot)>,
    root_q: Query<(&TState, &AttackDetectorClosest, &AttackDetectorTargeted)>
) {
    for (target_terminal, bang, to_root) in q.iter_mut() {
        if bang.is_active() { // Update target, while bang is inactive.
            continue;
        }
        target_update(target_terminal, to_root, &root_q);
    }
}

fn target_update(
    mut target_terminal: Mut<AttackTarget>,
    to_root: &ToBehaviourRoot,
    root_q: &Query<(&TState, &AttackDetectorClosest, &AttackDetectorTargeted)>
) {
    ref_caravan!(to_root::root_q((state, closest, targeted)););

    let state: TreeState = state.state();
    if state.contains(ATTACK_MOVE) {
        target_terminal.0 = closest.0;
    } else if state.contains(ATTACK_TARGET) {
        target_terminal.0 = targeted.0;
    } else {
        target_terminal.0 = closest.0;
    }
}

// Behaviour
#[derive(Component)]
pub struct AttackTimer(f32);
impl Default for AttackTimer {
    fn default() -> Self {
        Self(0.0)
    }
}

#[derive(Component)]
pub struct AttackTrigger{
    trigger_time: f32,
    triggered: bool,
}
impl AttackTrigger {
    pub fn new(trigger_at_time: f32) -> Self {
        return Self{
            trigger_time: trigger_at_time,
            triggered: false,
        }
    }
}
impl Default for AttackTrigger {
    fn default() -> Self {
        // Should not be defaulted
        Self { triggered: false, trigger_time: 0.0 }
    }
}

#[derive(Component)]
pub struct AttackEndTrigger(f32);
impl AttackEndTrigger {
    pub fn new(trigger_at_time: f32) -> Self {
        return Self(
            trigger_at_time
        )
    }
}
impl Default for AttackEndTrigger {
    fn default() -> Self {
        // Should not be defaulted
        Self(
            0.0
        )
    }
}

pub fn attack_timer_reset_sys(
    mut q: Query<(&mut AttackTimer, &Bang), Changed<Bang>>
) {
    for (mut timer, bang) in q.iter_mut() {
        if bang.is_active() {
            continue; // Change detection should be enough, but that isn't working for some reason. The bang is being updated every frame it seems.
        }
        timer.0 = 0.0;
    }
}

// This timer stuff could be extracted to being a standardised set of bang duration components.
pub fn attack_timer_sys(
    mut q: Query<(&mut AttackTimer, &Bang)>,
    time: Res<Time>,
) {
    for (mut timer, bang) in q.iter_mut() {
        if !bang.is_active() {
            continue;
        }

        timer.0 = timer.0 + time.delta_seconds();
    }
}

pub fn attack_reset_sys(
    mut q: Query<(&mut AttackTrigger, &Bang)>,
) {
    for (mut trigger, bang) in q.iter_mut() {
        if bang.is_active() {
            continue;
        }

        trigger.triggered = false;
    }
}

/// Expects direct attack terminal to be local.
pub fn attack_execution_sys(
    mut q: Query<(&mut DirectAttackBang, &mut AttackTrigger, &AttackTimer, &AttackTarget), Changed<AttackTimer>>,
) {
    for (mut attack_bang, mut trigger, timer, target) in q.iter_mut() {
        if trigger.triggered {
            continue;
        }

        if !(timer.0 >= trigger.trigger_time) {
            continue;
        }

        trigger.triggered = true;
        let Some(target) = target.0 else {
            continue;
        };

        attack_bang.bang(target);
    }
}

pub fn attack_end_sys(
    mut q: Query<(&AttackEndTrigger, &mut AttackTimer, &ToBehaviourRoot, &mut AttackTrigger), Changed<AttackTimer>>,
    mut root_q: Query<&mut TUnitIMCAMapper>,
) {
    for (end, timer, to_root, execute) in q.iter_mut() {
        if !(timer.0 >= end.0) {
            continue;
        }

        attack_end(timer, execute, to_root, &mut root_q);
    }
}

pub fn attack_end(
    mut timer: Mut<AttackTimer>,
    mut execute: Mut<AttackTrigger>,
    to_root: &ToBehaviourRoot,
    root_q: &mut Query<&mut TUnitIMCAMapper>,
) {
    ref_caravan!(to_root::root_q(mut imca_mapper););
    imca_mapper.0 = 3; // to Attack

    timer.0 = 0.0;
    execute.triggered = false;
}

#[derive(Component, Default)]
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