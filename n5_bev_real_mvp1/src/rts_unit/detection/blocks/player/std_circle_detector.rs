use bevy::prelude::*;

use crate::rts_unit::{
    *,
    blocks::Block,
    control::RTSUnitControl,
    unit_type::RTSTeam,
    detection::parts::*,
};

#[derive(Bundle)]
pub struct PlayerCircleDetectorBlock {
    pub to_root: ToRTSUnitRoot,
    pub transform: TransformBundle,

    pub detector: CircleIntersectUnitDetector,
    pub arbitrary_detection: ArbitraryUnitDetection,
    pub closest_detection: ClosestUnitDetection,

    pub target_detection: TargetUnitDetection,
    pub target_from: TargetFromCommandable,
}

#[derive(Clone, Copy)]
pub struct Parameters {
    pub position: Vec3,
    pub range: f32,
}


#[derive(Clone, Copy)]
pub struct EntityReferences {
    pub root: RTSUnit,
    pub control: RTSUnitControl, // Target is recieved from commandable
}

impl Block<PlayerCircleDetectorBlock, Parameters, EntityReferences> for PlayerCircleDetectorBlock {
    fn new_complete_bundle(params: Parameters, entity_references: EntityReferences) -> PlayerCircleDetectorBlock {
        return Self {
            to_root: entity_references.root,
            transform: TransformBundle{
                local: Transform{
                    translation: params.position,
                    ..default()
                }, ..default() },
            detector: CircleIntersectUnitDetector::new(params.range, RTSTeam::Enemy),
            arbitrary_detection: ArbitraryUnitDetection::new(),
            closest_detection: ClosestUnitDetection::new(),
            target_detection: TargetUnitDetection::new(),
            target_from: TargetFromCommandable::new(entity_references.control),
        }
    }
}