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
use crate::rts_unit::unit_type::player::prince::Prince;
use crate::rts_unit::{
    movement::Mover,
    ToRTSUnitRoot
};

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct HuntPrinceAutonomous;

fn update(
    behaviour_q: Query<(&ToRTSUnitRoot, &ToAttackArbitraryDetection), With<HuntPrinceAutonomous>>,
    detector_q: Query<&ArbitraryUnitDetection>,
    mut root_q: Query<(&mut Mover, &Transform)>,
    prince_q: Query<&Transform, With<Prince>>,
) {
    let prince = prince_q.get_single();
    if prince.is_err() {
        return;
    }
    let prince_transform = prince.unwrap();

    for (to_root, to_arb_detect) in behaviour_q.iter() {
        let root = to_root.root();
        let (mut mover, transform) = root_q.get_mut(root.0).unwrap();
        let position = transform.translation.truncate();
        let arb_detect = to_arb_detect.entity();
        let arb_detect = detector_q.get(arb_detect).unwrap();
        let detection = arb_detect.detection();
    
        if detection.is_none() {
            let waypoint = prince_transform.translation.truncate();
            let move_vec = (waypoint - position).normalize_or_zero();
            mover.input(move_vec);
        } else {
            mover.input(Vec2::ZERO);
        }
    }
}