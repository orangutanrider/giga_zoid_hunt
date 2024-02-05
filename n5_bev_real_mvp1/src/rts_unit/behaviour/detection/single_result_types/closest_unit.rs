use bevy::prelude::*;

use super::SingleResultDetection;
use crate::rts_unit::soul::RTSUnitSoulID;

#[derive(Component)]
pub struct ClosestUnitDetection {
    closest_unit: Option<RTSUnitSoulID>,
}
impl Default for ClosestUnitDetection {
    fn default() -> Self {
        return Self { closest_unit: None }
    }
}
impl ClosestUnitDetection {
    pub fn new() -> Self {
        return Self {
            closest_unit: None
        }
    }
}

impl SingleResultDetection for ClosestUnitDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoulID>,
    ) {
        self.closest_unit = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoulID> {
        return self.closest_unit;
    }
}