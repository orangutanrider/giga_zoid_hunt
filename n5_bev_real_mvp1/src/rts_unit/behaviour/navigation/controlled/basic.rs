use bevy::prelude::*;

use crate::rts_unit::behaviour::detection::single_result_types::{
    SingleResultDetection,
    arbitrary_unit::ArbitraryUnitDetection,
    target_unit::TargetUnitDetection,
};
use crate::rts_unit::behaviour::detection::to_detection::attack_detection::{
    ToAttackArbitraryDetection,
    ToAttackTargetDetection,
};
use crate::rts_unit::{
    movement::Mover,
    ToRTSUnitRoot
};
use crate::rts_unit::control::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            behaviour_update_arb,
            behaviour_update_target,
        ));
    }
}

#[derive(Component)]
pub struct BasicControlled;

fn behaviour_update_arb (
    behaviour_q: Query<(&ToRTSUnitRoot, &ToAttackArbitraryDetection), With<BasicControlled>>,
    detector_q: Query<&ArbitraryUnitDetection>,
    control_q: Query<&Commandable>,
    mut root_q: Query<(&mut Mover, &RTSUnitControlEntity, &Transform)>,
) {
    for (to_root, to_arb_detect) in behaviour_q.iter() {
        let root = to_root.root();
        let (mut mover, control_entity, transform) = root_q.get_mut(root.0).unwrap();
        let position = transform.translation.truncate();
        let commandable = control_q.get(control_entity.entity()).unwrap();
        let arb_detect = to_arb_detect.entity();
        let arb_detect = detector_q.get(arb_detect).unwrap();

        let order = commandable.current_order();
        if order.is_none() {
            mover.input(Vec2::ZERO); 
            return;
        }
        let order = order.unwrap();

        match order.order_type {
            OrderType::AttackTarget(_) => { },
            OrderType::Empty => {
                mover.input(Vec2::ZERO);
            },
            OrderType::PureMovement(pure_movement_order) => {
                follow_pure_move_order(mover, pure_movement_order, position);
            },
            OrderType::AttackMove(attack_move_order) => {
                follow_attack_move_order(arb_detect, mover, attack_move_order, position);
            },
        }
    }
}

fn behaviour_update_target (
    behaviour_q: Query<(&ToRTSUnitRoot, &ToAttackTargetDetection), With<BasicControlled>>,
    detector_q: Query<&TargetUnitDetection>,
    control_q: Query<&Commandable>,
    mut root_q: Query<(&mut Mover, &RTSUnitControlEntity, &Transform)>,

    transform_q: Query<&Transform>,
) {
    for (to_root, to_target_detect) in behaviour_q.iter() {
        let root = to_root.root();
        let (mut mover, control_entity, transform) = root_q.get_mut(root.0).unwrap();
        let position = transform.translation.truncate();
        let commandable = control_q.get(control_entity.entity()).unwrap();
        let target_detect = to_target_detect.entity();
        let target_detect = detector_q.get(target_detect).unwrap();

        let order = commandable.current_order();
        if order.is_none() {
            mover.input(Vec2::ZERO); 
            return;
        }
        let order = order.unwrap();

        match order.order_type {
            OrderType::Empty => { },
            OrderType::PureMovement(_) => { },
            OrderType::AttackMove(_) => { },
            OrderType::AttackTarget(attack_target_order) => {
                follow_attack_target_order(&transform_q, target_detect, mover, attack_target_order, position);
            },
        }
    }
}

fn follow_pure_move_order(
    mut mover: Mut<Mover>,
    order: PureMovementOrder,
    position: Vec2,
) {
    let move_vec = (order.waypoint - position).normalize_or_zero();
    mover.input(move_vec);
}

fn follow_attack_move_order(
    arb_detect: &ArbitraryUnitDetection,
    mut mover: Mut<Mover>,
    order: AttackMoveOrder,
    position: Vec2,
) {
    let detection = arb_detect.detection();

    if detection.is_none() {
        let move_vec = (order.waypoint - position).normalize_or_zero();
        mover.input(move_vec);
    } else {
        mover.input(Vec2::ZERO);
    }
}

fn follow_attack_target_order(
    transform_q: &Query<&Transform>,
    target_detect: &TargetUnitDetection,
    mut mover: Mut<Mover>,
    order: AttackTargetOrder,
    position: Vec2,
) {
    let detection = target_detect.detection();
    let target = order.target; // This and the value above could theoretically become out of sync.
    
    if target.is_none() {
        mover.input(Vec2::ZERO);
        return;
    }
    let target = target.unwrap();

    if detection.is_none() {
        let waypoint = transform_q.get(target.entity());
        let waypoint = waypoint.unwrap().translation.truncate();
        let move_vec = (waypoint - position).normalize_or_zero();
        mover.input(move_vec);
    } else {
        mover.input(Vec2::ZERO);
    }
}
