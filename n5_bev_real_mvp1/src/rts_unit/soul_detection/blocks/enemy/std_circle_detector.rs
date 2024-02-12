use bevy::prelude::*;

use crate::rts_unit::{
    block::SimpleBlock,
    unit_type::RTSTeam,
    detection::parts::*,
};

#[derive(Bundle)]
pub struct EnemyCircleDetectorBlock {
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

impl SimpleBlock<EnemyCircleDetectorBlock, Parameters> for EnemyCircleDetectorBlock {
    fn new_bundle(params: Parameters) -> EnemyCircleDetectorBlock {
        return Self {
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