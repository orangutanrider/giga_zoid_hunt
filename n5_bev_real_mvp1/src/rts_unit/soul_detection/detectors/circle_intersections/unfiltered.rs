use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::utils::HashMap;

use crate::rts_unit::soul_detection::parts::*;
use crate::rts_unit::{
    soul::*, 
    GetEntityRef
};

use super::*;

pub fn detector_update(
    mut detector_q: Query<(&mut CircleIntersectSoulDetector, &GlobalTransform), Without<AdditionalDetectorFilter>>, 
    collider_q: Query<&Collider>,
    rapier_context: Res<RapierContext>,
){
    for (mut detector, transform, ) in detector_q.iter_mut() {
        detector_detect(detector, transform, &collider_q, &rapier_context);
    }
}

fn detector_detect(
    mut detector: Mut<CircleIntersectSoulDetector>,
    transform: &GlobalTransform,
    collider_q: &Query<&Collider>,
    rapier_context: &Res<RapierContext>,
) {
    let position = transform.translation().truncate();

    // During detection outputs
    let mut entity_distances = HashMap::new();
    let mut target_output: Option<RTSUnitSoul> = None;

    // During detection processses
    let target = detector.target;
    if target.is_some() {
        let target = target.unwrap().entity();
        target_detect(&mut detector, &rapier_context, &collider_q, position, target, &mut target_output, &mut entity_distances);
    }
    else {
        detect(&mut detector, &rapier_context, &collider_q, position, &mut entity_distances);
    }

    // Post detection processes
    let closest_output = closest_unit_from_detection_results(entity_distances);
    let arbitrary_output = closest_output;

    // Post detection output
    detector.target_detection = target_output;
    detector.closest_detection = closest_output;
    detector.arbitrary_detection = arbitrary_output;
}

fn target_detect(
    detector: &mut Mut<CircleIntersectSoulDetector>,
    rapier_context: &Res<RapierContext>,
    collider_q: &Query<&Collider>,
    position: Vec2,
    target: Entity,
    target_output: &mut Option<RTSUnitSoul>,
    entity_distance_output: &mut HashMap<Entity, f32>,
) {
    let callback = |entity|{
        check_if_target_unit(target, entity, target_output);
        output_entity_distances(collider_q, entity, position, entity_distance_output);
        return true;
    };

    // Detect
    detector.detect_at(rapier_context, position, callback);   
}

fn detect(
    detector: &mut Mut<CircleIntersectSoulDetector>,
    rapier_context: &Res<RapierContext>,
    collider_q: &Query<&Collider>,
    position: Vec2,
    entity_distance_output: &mut HashMap<Entity, f32>,
) {
    let callback = |entity|{
        output_entity_distances(collider_q, entity, position, entity_distance_output);
        return true;
    };

    // Detect
    detector.detect_at(rapier_context, position, callback);   
}