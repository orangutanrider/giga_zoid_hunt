use bevy::prelude::*;

use super::SingleResultDetection;
use crate::rts_unit::RTSUnitID;

#[derive(Component)]
pub struct ClosestUnitDetection {
    closest_unit: Option<RTSUnitID>,
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
        &self,
        detection: Option<RTSUnitID>,
    ) {
        self.closest_unit = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitID> {
        return self.closest_unit;
    }
}