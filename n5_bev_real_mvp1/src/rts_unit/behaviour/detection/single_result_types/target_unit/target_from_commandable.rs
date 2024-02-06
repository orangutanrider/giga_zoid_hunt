use bevy::prelude::*;

use crate::rts_unit::soul::RTSUnitSoulID;
use crate::rts_unit::control::{
    prelude::*, 
    RTSUnitControlID
};
use super::TargetUnitDetection;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            target_from_commandable,
        ));
    }
}

#[derive(Component)]
pub struct TargetFromCommandable{
    control_entity: RTSUnitControlID,
}
impl Default for TargetFromCommandable {
    fn default() -> Self {
        return Self {
            control_entity: RTSUnitControlID::PLACEHOLDER,
        }
    }
}
impl TargetFromCommandable {
    pub fn new(control_entity: RTSUnitControlID) -> Self {
        return Self {
            control_entity,
        }
    }
}

fn target_from_commandable(
    mut detector_q: Query<(&mut TargetUnitDetection, &TargetFromCommandable)>,
    commandable_q: Query<&Commandable>,
) {
    for (mut detection, from_commandable) in detector_q.iter_mut() {
        let control_entity = from_commandable.control_entity.entity();
        let commandable = commandable_q.get(control_entity);
        let commandable = commandable.unwrap();
    
        let order = commandable.current_order();
        let mut target: Option<RTSUnitSoulID> = None;
        match order.order_type {
            OrderType::PureMovement(_) => {},
            OrderType::AttackMove(_) => {},
            OrderType::Empty => {},
            OrderType::AttackTarget(attack_target) => {
                target = attack_target.target;
            },
        }

        detection.set_target(target)
    }
}