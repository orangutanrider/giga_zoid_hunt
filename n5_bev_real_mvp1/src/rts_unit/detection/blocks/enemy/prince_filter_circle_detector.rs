use bevy::prelude::*;

use crate::rapier_config::groups::P_PRINCE;

use crate::rts_unit::{
    block::SimpleBlock,
    unit_type::RTSTeam,
    detection::parts::*,
};

#[derive(Bundle)]
pub struct EnemyPrinceFilterCircleDetectorBlock {
    pub transform: TransformBundle,

    pub detector: CircleIntersectUnitDetector,
    pub arbitrary_detection: ArbitraryUnitDetection,
    pub filter: AdditionalDetectorFilter,
}

#[derive(Clone, Copy)]
pub struct Parameters {
    pub position: Vec3,
    pub range: f32,
}

impl SimpleBlock<EnemyPrinceFilterCircleDetectorBlock, Parameters> for EnemyPrinceFilterCircleDetectorBlock {
    fn new_bundle(params: Parameters) -> EnemyPrinceFilterCircleDetectorBlock {
        return Self {
            transform: TransformBundle{
                local: Transform{
                    translation: params.position,
                    ..default()
                }, ..default() },
            detector: CircleIntersectUnitDetector::new(params.range, RTSTeam::Enemy),
            arbitrary_detection: ArbitraryUnitDetection::new(),
            filter: AdditionalDetectorFilter::new(P_PRINCE),
        }
    }
}