use bevy::prelude::*;

use crate::rts_unit::{
    *,
    movement::*,
};
use super::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, basic_pure_move_update);
    }
}

#[derive(Component)]
struct PureMoveToMover;
impl TypeIdGet for PureMoveToMover { }
impl EntityReferenceFlag<2, RTSUnitRoot> for PureMoveToMover {
    const REFERENCE_PATH: [TypeId; 2] = [ToRoot::TYPE_ID, RTSUnitRoot::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Mutable;
}

#[derive(Component)]
pub struct BasicPureMoveOrderBehaviour;

fn basic_pure_move_update(
    behaviour_q: Query<(&TFollowedOrder, &ToRoot, &GlobalTransform), (With<BasicPureMoveOrderBehaviour>, With<PureMoveToMover>)>,
    root_q: Query<&TMover>,
) {
    for (terminal, to_root, transform) in behaviour_q.iter() {
        basic_pure_move(root_q, terminal, to_root, transform);
    }
}

fn basic_pure_move(
    root_q: Query<&TMover>,
    terminal: &TFollowedOrder, 
    to_root: &ToRoot, 
    transform: &GlobalTransform,
) {
    let order = terminal.read();
    let Some(order) = order else {
        return; // if no order, return (stopping is handled by another component)
    };

    match order.order_type {
        OrderType::PureMovement(move_order) => {
            let position = transform.translation().truncate();
            follow_move_order(root_q, to_root, move_order, position);
        },
        OrderType::AttackTarget(_) => {},
        OrderType::AttackMove(_) => {},
    }
}

fn follow_move_order(
    root_q: Query<&TMover>,
    to_root: &ToRoot,
    move_order: PureMovementOrder,
    position: Vec2,
) {    
    // Follow reference path
    let root = to_root.entity();
    let mover = root_q.get(root);
    let Ok(mover) = mover else {
        PureMoveToMover::print_err_descript(1, "failed at getting TMover from the entity.");
        return;
    };

    let move_vec = (move_order.waypoint - position).normalize_or_zero();
    mover.input(move_vec);
}