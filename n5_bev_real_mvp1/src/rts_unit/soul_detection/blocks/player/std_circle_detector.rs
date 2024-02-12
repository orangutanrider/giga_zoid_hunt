use bevy::prelude::*;

use crate::rts_unit::{
    *,
    block::IntegratedBlock,
    unit_type::RTSTeam,
    detection::parts::*,
};

#[derive(Bundle)]
pub struct PlayerCircleDetectorBlock {
    pub to_root: ToRoot,
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
    pub root: RTSUnitRoot, // root -> control (target is recieved via commandable)
}

impl IntegratedBlock<PlayerCircleDetectorBlock, Parameters, EntityReferences> for PlayerCircleDetectorBlock {
    fn new_bundle(params: Parameters, entity_references: EntityReferences) -> PlayerCircleDetectorBlock {
        return Self {
            to_root: ToRoot::new(entity_references.root.entity()),
            transform: TransformBundle{
                local: Transform{
                    translation: params.position,
                    ..default()
                }, ..default() },
            detector: CircleIntersectUnitDetector::new(params.range, RTSTeam::Enemy),
            arbitrary_detection: ArbitraryUnitDetection::new(),
            closest_detection: ClosestUnitDetection::new(),
            target_detection: TargetUnitDetection::new(),
            target_from: TargetFromCommandable,
        }
    }

    fn spawn_complete(
        commands: &mut Commands,
        params: Parameters,
        entity_references: EntityReferences,
    ) -> Entity  {
        let entity = commands.spawn( Self::new_bundle(params, entity_references)).id();
        commands.entity(entity_references.root.entity()).push_children(&[entity]);
        return entity
    }
}