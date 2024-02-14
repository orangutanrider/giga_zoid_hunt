use bevy::prelude::*;

use crate::rts_unit::{
    *,
    soul::RTSUnitSoul,
    control::{
        parts::*,
        RootToControl,
        RTSUnitControl,
} };

use super::TTargetSoulDetection;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            target_from_commandable_update,
        ));
    }
}

#[derive(Component)]
pub struct TargetFromCommandable;
impl TypeIdGet for TargetFromCommandable { }
impl EntityReferenceFlag<3, RTSUnitControl> for TargetFromCommandable {
    const REFERENCE_PATH: [TypeId; 3] = [ToRoot::TYPE_ID, RootToControl::TYPE_ID, RTSUnitControl::TYPE_ID];
    const REF_TYPE: EntityRefFlagRefType = EntityRefFlagRefType::Immutable;
}

fn target_from_commandable_update(
    mut detector_q: Query<(&mut TTargetSoulDetection, &ToRoot), With<TargetFromCommandable>>,
    root_q: Query<&RootToControl>,
    control_q: Query<&Commandable>,
) {
    for (mut detection, to_root) in detector_q.iter_mut() {
        target_from_commandable(detection, to_root, root_q, control_q);
    }
}

fn target_from_commandable(
    mut detection: Mut<TTargetSoulDetection>, 
    to_root: &ToRoot,
    root_q: Query<&RootToControl>,
    control_q: Query<&Commandable>,
) {
    // Follow reference path
    let root = to_root.entity();
    let to_control = root_q.get(root);
    let Ok(to_control) = to_control else {
        TargetFromCommandable::print_err(1);
        return;
    };
    let control = to_control.entity();
    let commandable = control_q.get(control);
    let Ok(commandable) = commandable else {
        TargetFromCommandable::print_err_descript(2, "failed at getting commandable from the entity.");
        return;
    };

    // Set target to none, if no order
    let mut target: Option<RTSUnitSoul> = None;
    let order = commandable.current_order();
    let Some(order) = order else {
        detection.set_target(target);
        return
    };

    // Set target to some, if the order is there
    match order.order_type {
        OrderType::PureMovement(_) => {},
        OrderType::AttackMove(_) => {},
        OrderType::AttackTarget(attack_target) => {
            target = attack_target.target;
        },
    }
    detection.set_target(target)
}