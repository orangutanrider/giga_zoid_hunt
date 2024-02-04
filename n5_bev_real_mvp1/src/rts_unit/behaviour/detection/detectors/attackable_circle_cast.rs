use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::utils::HashMap;

use crate::rapier_config::prelude::{
    E_ATTACKABLE_FILTER,
    P_ATTACKABLE_FILTER,
};
use crate::rts_unit::behaviour::detection::single_result_types::closest_unit;
use crate::rts_unit::behaviour::detection::single_result_types::{
    closest_unit::ClosestUnitDetection,
    target_unit::TargetUnitDetection,
};
use crate::rts_unit::unit_types::RtsTeam;
use crate::rts_unit::{RTSUnitID, RTSUnitSubEntity};

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detector_update);
    }
}

#[derive(Component)]
pub struct CircleCastUnitDetector {
    radius: f32,
    team: RtsTeam,
}
impl CircleCastUnitDetector {
    pub fn new(
        radius: f32,
        team: RtsTeam,
    ) -> Self {
        return Self { 
            radius,
            team,
        }
    }
}

impl CircleCastUnitDetector {
    fn filter(&self) -> QueryFilter {
        match self.team {
            RtsTeam::Enemy => { return P_ATTACKABLE_FILTER },
            RtsTeam::Player => { return E_ATTACKABLE_FILTER },
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
    mut target_output: &mut Option<Entity>,
) {
    if entity == target_unit {
        target_output = &mut Some(entity);
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
    q: &Query<&RTSUnitSubEntity>,
    rapier_context: &Res<RapierContext>,
    collider_q: &Query<&Collider>,
    position: Vec2,
    entity_distances: HashMap<Entity, f32>,
) -> Option<RTSUnitID> {
    let closest_entity = closest_entity_from_detection_results(rapier_context, collider_q, position, entity_distances);
    if closest_entity.is_none() { return None }
    let closest_entity = closest_entity.unwrap();

    let to_root = q.get(closest_entity);
    let to_root = to_root.unwrap();
    return Some(to_root.root());
}

/// Closest Unit Results Processing
fn closest_entity_from_detection_results(
    rapier_context: &Res<RapierContext>,
    collider_q: &Query<&Collider>,
    position: Vec2,
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
    mut detector_q: Query<(
        &mut CircleCastUnitDetector, &Transform, // Detector relevant
        &mut ClosestUnitDetection, &mut TargetUnitDetection // Detection output
    )>, 
    collider_q: Query<&Collider>,
    sub_entity_q: Query<&RTSUnitSubEntity>,
    rapier_context: Res<RapierContext>,
){
    for (
        detector, transform, 
        closest_unit_detection, target_unit_detection
    ) in detector_q.iter_mut() {
        let position = transform.translation.truncate();

        // Detection outputs
        let mut entity_distances = HashMap::new();
        let mut target_output: Option<Entity> = None;

        // During detection processses
        
        let callback = |entity| -> bool {
            check_if_target_unit(target_unit, entity, &mut target_output);
            output_entity_distances(&collider_q, entity, position, &mut entity_distances);
            return true;
        };

        detector.detect_at(&rapier_context, position, callback);

        // Post detection processes
        let closest_unit = closest_unit_from_detection_results(&sub_entity_q, &rapier_context, &collider_q, position, entity_distances);
    }
}