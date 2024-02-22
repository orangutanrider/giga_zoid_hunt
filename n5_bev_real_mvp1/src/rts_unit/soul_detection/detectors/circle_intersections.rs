mod unfiltered;
mod filtered;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::utils::HashMap;

use crate::rapier_config::prelude::{
    E_DETECTABLE_FILTER,
    P_DETECTABLE_FILTER,
};
use crate::rts_unit::{
    soul_detection::parts::*,
    soul::*,
    unit_type::RTSTeam,
};

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            filtered::detector_update,
            unfiltered::detector_update,
            store_detection_target,
            stored_target_output_to_detection,
            stored_closest_output_to_detection,
            stored_arbitrary_output_to_detection,
        ));
    }
}

#[derive(Component)]
pub struct CircleIntersectSoulDetector {
    radius: f32,
    target_team: RTSTeam,

    target: Option<RTSUnitSoul>, // Input
    target_detection: Option<RTSUnitSoul>, // Output
    closest_detection: Option<RTSUnitSoul>, // Output
    arbitrary_detection: Option<RTSUnitSoul>, // Output
}
impl CircleIntersectSoulDetector {
    pub fn new(
        radius: f32,
        target_team: RTSTeam,
    ) -> Self {
        return Self { 
            radius,
            target_team,

            target: None,
            target_detection: None,
            closest_detection: None,
            arbitrary_detection: None,
        }
    }
}

impl CircleIntersectSoulDetector {
    fn filter(&self) -> QueryFilter {
        match self.target_team {
            RTSTeam::Player => {
                return P_DETECTABLE_FILTER;
            },
            RTSTeam::Enemy => {
                return E_DETECTABLE_FILTER;
            },
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
    target_output: &mut Option<RTSUnitSoul>,
) {
    if entity == target_unit {
        *target_output = Some(RTSUnitSoul::new(entity));
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
) -> Option<RTSUnitSoul> {
    let closest_entity = closest_entity_from_detection_results(entity_distances);
    if closest_entity.is_none() { return None }
    let closest_entity = closest_entity.unwrap();

    return Some(RTSUnitSoul::new(closest_entity));
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

/// If detector has a target detection with it, it'll try to get the target from that 
fn store_detection_target(
    mut detector_q: Query<(&mut CircleIntersectSoulDetector, &TTargetSoulDetection)>, 
) {
    for (mut detector, detection) in detector_q.iter_mut() {
        detector.target = detection.target();
    }
}

/// If detector has a target detection with it, it'll try to output to it
fn stored_target_output_to_detection( 
    mut detector_q: Query<(&CircleIntersectSoulDetector, &mut TTargetSoulDetection)>, 
) {
    for (detector, mut detection) in detector_q.iter_mut() {
        detection.set_detection(detector.target_detection);
    }
}

/// If detector has a closest detection with it, it'll try to output to it
fn stored_closest_output_to_detection(
    mut detector_q: Query<(&CircleIntersectSoulDetector, &mut TClosestSoulDetection)>, 
) {
    for (detector, mut detection) in detector_q.iter_mut() {
        detection.set_detection(detector.closest_detection);
    }
}

/// If detector has a arbitrary detection with it, it'll try to output to it
fn stored_arbitrary_output_to_detection(
    mut detector_q: Query<(&CircleIntersectSoulDetector, &mut TArbitrarySoulDetection)>, 
) {
    for (detector, mut detection) in detector_q.iter_mut() {
        detection.set_detection(detector.arbitrary_detection);
    }
}