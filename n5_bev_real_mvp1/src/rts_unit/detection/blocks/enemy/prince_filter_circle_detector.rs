use bevy::prelude::*;

use crate::rapier_config::groups::P_PRINCE;

use crate::rts_unit::{
    *,
    blocks::Block,
    unit_type::RTSTeam,
    detection::parts::*,
};

#[derive(Bundle)]
pub struct EnemyPrinceFilterCircleDetectorBlock {
    pub to_root: ToRTSUnitRoot,
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

#[derive(Clone, Copy)]
pub struct EntityReferences {
    pub root: RTSUnit,
}

impl Block<EnemyPrinceFilterCircleDetectorBlock, Parameters, EntityReferences> for EnemyPrinceFilterCircleDetectorBlock {
    fn new_complete_bundle(params: Parameters, entity_references: EntityReferences) -> EnemyPrinceFilterCircleDetectorBlock {
        return Self {
            to_root: entity_references.root,
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