use std::process::Command;

use bevy::{prelude::*, transform};

use crate::rts_unit::behaviour::detection::ClosestUnitDetection;
use crate::rts_unit::control::prelude::*;
use crate::rts_unit::{
    movement::Mover,
    RTSUnitSubEntity
};

#[derive(Component)]
struct BasicControlled;

fn behaviour_update (
    behaviour_q: Query<(&RTSUnitSubEntity, &ClosestUnitDetection), With<BasicControlled>>,
    control_q: Query<&Commandable>,
    mut root_q: Query<(&mut Mover, &RTSUnitControlEntity, &Transform)>,
) {
    for (sub_entity, detection) in behaviour_q.iter() {
        let root = sub_entity.root();
        let (mover, control_entity, transform) = root_q.get_mut(root.0).unwrap();
        let commandable = control_q.get(control_entity.entity()).unwrap();

        let order = commandable.current_order();
        let position = transform.translation.truncate();
        match order.order_type {
            OrderType::Empty => {},
            OrderType::PureMovement(pure_movement_order) => {
                follow_pure_move_order(mover, pure_movement_order, position);
            },
            OrderType::AttackMove(attack_move_order) => {

            },
            OrderType::AttackTarget(attack_target_order) => {

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
    detection: &ClosestUnitDetection,
    mut mover: Mut<Mover>,
    order: PureMovementOrder,
    position: Vec2,
) {
    let closest_unit = detection.closest_unit_in_range();

    if closest_unit.is_none() {
        let move_vec = (order.waypoint - position).normalize_or_zero();
        mover.input(move_vec);
    } else {
        mover.input(Vec2::ZERO);
    }
}

fn follow_attack_target_order() {

}
