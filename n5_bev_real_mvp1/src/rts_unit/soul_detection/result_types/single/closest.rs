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
/// Closest RTSUnitSoul Detection Terminal
pub struct TClosestSoulDetection {
    detection: Option<RTSUnitSoul>,
}
impl Default for TClosestSoulDetection {
    fn default() -> Self {
        return Self { detection: None }
    }
}
impl TClosestSoulDetection {
    pub fn new() -> Self {
        return Self {
            detection: None
        }
    }
}

impl SingleResultDetection for TClosestSoulDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoul>,
    ) {
        self.detection = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoul> {
        return self.detection;
    }
}