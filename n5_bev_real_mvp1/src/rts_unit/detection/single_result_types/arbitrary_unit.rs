use bevy::prelude::*;

use super::SingleResultDetection;
use crate::rts_unit::soul::RTSUnitSoul;

#[derive(Component)]
pub struct ArbitraryUnitDetection {
    unit: Option<RTSUnitSoul>,
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