use bevy::prelude::*;

use crate::rts_unit::{
    *,
    soul::RTSUnitSoul,
};
use super::SingleResultDetection;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct ArbitraryDetector(Entity);
entity_ref_impls!(ArbitraryDetector, SelfEntity);

#[derive(Component)]
pub struct RootToArbitraryDetector(Entity);
entity_ref_impls!(RootToArbitraryDetector, ChildEntity);

#[derive(Component)]
pub struct ChildToArbitraryDetector(Entity);
entity_ref_impls!(ChildToArbitraryDetector, ParentEntity);

#[derive(Component)]
/// Arbitrary RTSUnitSoul Detection Terminal
pub struct TArbitrarySoulDetection {
    detection: Option<RTSUnitSoul>,
}
impl Default for TArbitrarySoulDetection {
    fn default() -> Self {
        return Self { detection: None }
    }
}
impl TArbitrarySoulDetection {
    pub fn new() -> Self {
        return Self {
            detection: None
        }
    }
}

impl SingleResultDetection for TArbitrarySoulDetection {
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