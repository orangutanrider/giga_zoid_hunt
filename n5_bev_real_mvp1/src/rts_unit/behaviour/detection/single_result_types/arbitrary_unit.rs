use bevy::prelude::*;

use super::SingleResultDetection;
use crate::rts_unit::RTSUnitID;

#[derive(Component)]
pub struct ArbitraryUnitDetection {
    unit: Option<RTSUnitID>,
}
impl Default for ArbitraryUnitDetection {
    fn default() -> Self {
        return Self { unit: None }
    }
}
impl ArbitraryUnitDetection {
    pub fn new() -> Self {
        return Self {
            unit: None
        }
    }
}

impl SingleResultDetection for ArbitraryUnitDetection {
    fn set_detection(
        &self,
        detection: Option<RTSUnitID>,
    ) {
        self.unit = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitID> {
        return self.unit;
    }
}