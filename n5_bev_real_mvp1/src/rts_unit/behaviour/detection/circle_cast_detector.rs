use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::utils::HashMap;

use crate::rapier_config::prelude::{
    E_DETECTABLE_FILTER,
    P_DETECTABLE_FILTER,
};
use super::single_result_types::{
    SingleResultDetection,
    closest_unit::ClosestUnitDetection,
    target_unit::TargetUnitDetection,
    arbitrary_unit::ArbitraryUnitDetection,
};
use crate::rts_unit::{
    unit_types::RtsTeam,
    soul::RTSUnitSoulID,
};

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            detector_update,
            stored_target_output_to_detection,
            stored_closest_output_to_detection,
            stored_arbitrary_output_to_detection,
        ));
    }
}

#[derive(Component)]
pub struct CircleCastUnitDetector {
    radius: f32,
    team: RtsTeam,

    target: Option<RTSUnitSoulID>, // Input
    target_detection: Option<RTSUnitSoulID>, // Output
    closest_detection: Option<RTSUnitSoulID>, // Output
    arbitrary_detection: Option<RTSUnitSoulID>, // Output
}
impl CircleCastUnitDetector {
    pub fn new(
        radius: f32,
        team: RtsTeam,
    ) -> Self {
        return Self { 
            radius,
            team,

            target: None,
            target_detection: None,
            closest_detection: None,
            arbitrary_detection: None,
        }
    }
}

impl CircleCastUnitDetector {
    fn filter(&self) -> QueryFilter {
        match self.team {
            RtsTeam::Enemy => { return P_DETECTABLE_FILTER },
            RtsTeam::Player => { return E_DETECTABLE_FILTER },
        }
    }

    fn detect_at(        
        &self,
        rapier_context: &Res<RapierContext>,
        position: Vec2,
        callback: impl FnMut(Entity) -> bool,
    ) {
        let shape = Collider::ball(self.radius);
        rapier_context.intersections_with_shape(
            position, 
            0.0, 
            &shape, 
            self.filter(), 
            callback
        );
    }
}

/// Target Unit Processing
fn check_if_target_unit(
    target_unit: Entity, // to be replaced with a attackable wraper around the entity
    entity: Entity,
    target_output: &mut Option<RTSUnitSoulID>,
) {
    if entity == target_unit {
        *target_output = Some(RTSUnitSoulID::new(entity));
    }
}

/// Closest Unit Processing
fn output_entity_distances(
    collider_q: &Query<&Collider>,
    entity: Entity,
    location: Vec2,
    entity_distance_output: &mut HashMap<Entity, f32>,
) -> bool {
    let mut err = false;
    let err_collider = Collider::ball(0.0);
    let collider = collider_q.get(entity);
    let collider = collider.unwrap_or_else(|_| {
        err = true;
        return &err_collider;
    });
    if err { return false; } // If the system failed to get a collider

    let distance = collider.distance_to_local_point(location, true);
    entity_distance_output.insert(entity, distance);
    return true;
}

/// Closest Unit Results Processing
fn closest_unit_from_detection_results(
    entity_distances: HashMap<Entity, f32>,
) -> Option<RTSUnitSoulID> {
    let closest_entity = closest_entity_from_detection_results(entity_distances);
    if closest_entity.is_none() { return None }
    let closest_entity = closest_entity.unwrap();

    return Some(RTSUnitSoulID::new(closest_entity));
}

/// Closest Unit Results Processing
fn closest_entity_from_detection_results(
    entity_distances: HashMap<Entity, f32>,
) -> Option<Entity> {
    // If no detected
    if entity_distances.is_empty() {
        return None;
    }

    // Find closest
    let mut output_entity = Entity::PLACEHOLDER;
    let mut lowest_distance = f32::MAX;
    for (entity, distance) in entity_distances.iter() {
        if distance < &lowest_distance {
            output_entity = *entity;
            lowest_distance = *distance;
        }
    }
    
    return Some(output_entity)
}

fn detector_update(
    mut detector_q: Query<(&mut CircleCastUnitDetector, &Transform)>, 
    collider_q: Query<&Collider>,
    rapier_context: Res<RapierContext>,
){
    for (mut detector, transform, ) in detector_q.iter_mut() {
        let position = transform.translation.truncate();

        // During detection outputs
        let mut entity_distances = HashMap::new();
        let mut target_output: Option<RTSUnitSoulID> = None;

        // During detection processses
        let target_unit = detector.target;
        if target_unit.is_some() {
            let target_unit = target_unit.unwrap().entity();
            let callback = |entity|{
                check_if_target_unit(target_unit, entity, &mut target_output);
                output_entity_distances(&collider_q, entity, position, &mut entity_distances);
                return true;
            };

            // Detect
            detector.detect_at(&rapier_context, position, callback);   
        }
        else {
            let callback = |entity|{
                output_entity_distances(&collider_q, entity, position, &mut entity_distances);
                return true;
            };

            // Detect
            detector.detect_at(&rapier_context, position, callback);   
        }

        // Post detection processes
        let closest_output = closest_unit_from_detection_results(entity_distances);
        let arbitrary_output = closest_output;

        // Post detection output
        detector.target_detection = target_output;
        detector.closest_detection = closest_output;
        detector.arbitrary_detection = arbitrary_output;
    }
}

/// If detector has a target detection with it, it'll try to output to it
fn stored_target_output_to_detection( 
    mut detector_q: Query<(&CircleCastUnitDetector, &mut TargetUnitDetection)>, 
) {
    for (detector, mut detection) in detector_q.iter_mut() {
        detection.set_detection(detector.target_detection);
    }
}

/// If detector has a closest detection with it, it'll try to output to it
fn stored_closest_output_to_detection(
    mut detector_q: Query<(&CircleCastUnitDetector, &mut ClosestUnitDetection)>, 
) {
    for (detector, mut detection) in detector_q.iter_mut() {
        detection.set_detection(detector.closest_detection);
    }
}

/// If detector has a arbitrary detection with it, it'll try to output to it
fn stored_arbitrary_output_to_detection(
    mut detector_q: Query<(&CircleCastUnitDetector, &mut ArbitraryUnitDetection)>, 
) {
    for (detector, mut detection) in detector_q.iter_mut() {
        detection.set_detection(detector.arbitrary_detection);
    }
}