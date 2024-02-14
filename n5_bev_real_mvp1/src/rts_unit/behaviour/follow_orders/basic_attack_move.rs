use bevy::prelude::*;

use crate::rts_unit::{
    *,
    movement::*,
    soul_detection::parts::*,
};
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, basic_attack_move_update);
    }
}

#[derive(Component)]
struct AttackMoveToMover;
impl TypeIdGet for AttackMoveToMover { }
impl EntityReferenceFlag<2, RTSUnitRoot> for AttackMoveToMover {
    const REFERENCE_PATH: [TypeId; 2] = [ToRoot::TYPE_ID, RTSUnitRoot::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Mutable;
}

#[derive(Component)]
struct AttackMoveGetArbDetection;
impl TypeIdGet for AttackMoveGetArbDetection { }
impl EntityReferenceFlag<3, ArbitraryDetector> for AttackMoveGetArbDetection {
    const REFERENCE_PATH: [TypeId; 3] = [ToRoot::TYPE_ID, RootToArbitraryDetector::TYPE_ID, ArbitraryDetector::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Immutable;
}

#[derive(Component)]
pub struct BasicAttackMoveOrderBehaviour;

fn basic_attack_move_update(
    behaviour_q: Query<
        (&TFollowedOrder, &ToRoot, &GlobalTransform), 
        (With<BasicAttackMoveOrderBehaviour>, With<AttackMoveToMover>, With<AttackMoveGetArbDetection>)
    >,
    root_q: Query<(&TMover, &RootToArbitraryDetector)>,
    detector_q: Query<&TArbitrarySoulDetection>,
) {
    for (terminal, to_root, transform) in behaviour_q.iter() {
        basic_attack_move(root_q, detector_q, terminal, to_root, transform);
    }
}

fn basic_attack_move(
    root_q: Query<(&TMover, &RootToArbitraryDetector)>,
    detector_q: Query<&TArbitrarySoulDetection>,
    terminal: &TFollowedOrder, 
    to_root: &ToRoot, 
    transform: &GlobalTransform,
) {
    let order = terminal.read();
    let Some(order) = order else {
        return; // if no order, return (stopping is handled by another component)
    };

    match order.order_type {
        OrderType::PureMovement(_) => { },
        OrderType::AttackTarget(_) => { },
        OrderType::AttackMove(move_order) => {
            let position = transform.translation().truncate();
            follow_attack_move_order(root_q, detector_q, to_root, move_order, position);
        },
    }
}

fn follow_attack_move_order(
    root_q: Query<(&TMover, &RootToArbitraryDetector)>,
    detector_q: Query<&TArbitrarySoulDetection>,
    to_root: &ToRoot,
    move_order: AttackMoveOrder,
    position: Vec2,
) {    
    // Follow reference path
    let root = to_root.entity();
    let result = root_q.get(root);
    let Ok((mover, detector)) = result else {
        AttackMoveToMover::print_err_descript(1, "failed at getting either TMover or RootToArbitraryDetector from the entity.");
        return;
    };
    let detector = detector.entity();
    let detector = detector_q.get(detector);
    let Ok(detector) = detector else {
        AttackMoveGetArbDetection::print_err_descript(1, "failed at getting ArbitrarySoulDetection");
        return;
    };

    // Get detection, if there is a detection, stop moving
    let detection = detector.detection();
    if detection.is_some() {
        mover.input(Vec2::ZERO);
        return;
    }

    // move
    let move_vec = (move_order.waypoint - position).normalize_or_zero();
    mover.input(move_vec);
}