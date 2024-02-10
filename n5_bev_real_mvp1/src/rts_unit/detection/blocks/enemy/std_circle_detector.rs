use bevy::prelude::*;

use crate::rts_unit::{
    *,
    blocks::Block,
    unit_type::RTSTeam,
    detection::parts::*,
};

#[derive(Bundle)]
pub struct EnemyCircleDetectorBlock {
    pub to_root: ToRTSUnitRoot,
    pub transform: TransformBundle,

    pub detector: CircleIntersectUnitDetector,
    pub arbitrary_detection: ArbitraryUnitDetection,
    pub closest_detection: ClosestUnitDetection,
}

#[derive(Clone, Copy)]
pub struct Parameters {
    pub position: Vec3,
    pub range: f32,
}


#[derive(Clone, Copy)]
pub struct EntityReferences {
    pub root: RTSUnit,
}

impl Block<EnemyCircleDetectorBlock, Parameters, EntityReferences> for EnemyCircleDetectorBlock {
    fn new_complete_bundle(params: Parameters, entity_references: EntityReferences) -> EnemyCircleDetectorBlock {
        return Self {
            to_root: entity_references.root,
            transform: TransformBundle{
                local: Transform{
                    translation: params.position,
                    ..default()
                }, ..default() },
            detector: CircleIntersectUnitDetector::new(params.range, RTSTeam::Player),
            arbitrary_detection: ArbitraryUnitDetection::new(),
            closest_detection: ClosestUnitDetection::new(),
        }
    }
}