use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::utils::HashMap;

use crate::rapier_config::prelude::{
    E_ATTACKABLE_FILTER,
    P_ATTACKABLE_FILTER,
};
use crate::rts_unit::unit_types::RtsTeam;
use crate::rts_unit::RTSUnitID;
use super::ClosestUnitDetection;

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
    closest_unit: Option<RTSUnitID>,
}
impl CircleCastUnitDetector {
    pub fn new(
        radius: f32,
        team: RtsTeam,
    ) -> Self {
        return Self { 
            radius,
            team,
            closest_unit: None, 
        }
    }
}

impl ClosestUnitDetection for CircleCastUnitDetector {
    fn closest_unit_in_range(&self) -> Option<RTSUnitID> {
        return self.closest_unit;
    }
}

impl CircleCastUnitDetector {
    fn filter(&self) -> QueryFilter {
        match self.team {
            RtsTeam::Enemy => { return P_ATTACKABLE_FILTER },
            RtsTeam::Player => { return E_ATTACKABLE_FILTER },
        }
    }

    fn detect_closest_at(
        &self,
        rapier_context: &Res<RapierContext>,
        collider_q: &Query<&Collider>,
        position: Vec2,
    ) -> Option<RTSUnitID> {
        // On each detection, do
        let mut distances_of_detected: HashMap<Entity, f32> = HashMap::new();
        let callback = |entity: Entity| -> bool {
            insert_entity_distance_from_location(&collider_q, entity, position, &mut distances_of_detected)
        };
        
        // Detect
        let shape = Collider::ball(self.radius);
        rapier_context.intersections_with_shape(
            position, 
            0.0, 
            &shape, 
            self.filter(), 
            callback
        );

        // If no detected
        if distances_of_detected.is_empty() {
            return None;
        }

        // Find closest
        let mut output_entity = Entity::PLACEHOLDER;
        let mut lowest_distance = f32::MAX;
        for (entity, distance) in distances_of_detected.iter() {
            if distance < &lowest_distance {
                output_entity = *entity;
                lowest_distance = *distance;
            }
        }
        
        return Some(RTSUnitID(output_entity))
    }
}

fn insert_entity_distance_from_location(
    collider_q: &Query<&Collider>,
    entity: Entity,
    location: Vec2,
    output: &mut HashMap<Entity, f32>,
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
    output.insert(entity, distance);
    return true;
}

fn detector_update(
    mut detector_q: Query<(&mut CircleCastUnitDetector, &Transform)>,
    collider_q: Query<&Collider>,
    rapier_context: Res<RapierContext>,
){
    for (mut detector, transform) in detector_q.iter_mut() {
        let position = transform.translation.truncate();
        let detection_result = detector.detect_closest_at(&rapier_context, &collider_q, position);
        detector.closest_unit = detection_result;
    }
}