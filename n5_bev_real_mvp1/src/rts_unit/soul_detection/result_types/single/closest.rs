use bevy::prelude::*;

use crate::rts_unit::{
    *,
    soul::RTSUnitSoul,
};
use super::SingleResultDetection;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct ClosestDetector(Entity);
entity_ref_impls!(ClosestDetector, SelfEntity);

#[derive(Component)]
pub struct RootToClosestDetector(Entity);
entity_ref_impls!(RootToClosestDetector, ChildEntity);

#[derive(Component)]
pub struct ChildToClosestDetector(Entity);
entity_ref_impls!(ChildToClosestDetector, ParentEntity);

#[derive(Component)]
pub struct ClosestSoulDetection {
    closest_unit: Option<RTSUnitSoul>,
}
impl Default for ClosestSoulDetection {
    fn default() -> Self {
        return Self { closest_unit: None }
    }
}
impl ClosestSoulDetection {
    pub fn new() -> Self {
        return Self {
            closest_unit: None
        }
    }
}

impl SingleResultDetection for ClosestSoulDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoul>,
    ) {
        self.closest_unit = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoul> {
        return self.closest_unit;
    }
}