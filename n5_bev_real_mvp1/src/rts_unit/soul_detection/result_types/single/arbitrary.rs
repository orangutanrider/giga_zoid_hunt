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
pub struct ArbitrarySoulDetection {
    unit: Option<RTSUnitSoul>,
}
impl Default for ArbitrarySoulDetection {
    fn default() -> Self {
        return Self { unit: None }
    }
}
impl ArbitrarySoulDetection {
    pub fn new() -> Self {
        return Self {
            unit: None
        }
    }
}

impl SingleResultDetection for ArbitrarySoulDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoul>,
    ) {
        self.unit = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoul> {
        return self.unit;
    }
}