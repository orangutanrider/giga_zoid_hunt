use bevy::prelude::*;

use super::SingleResultDetection;
use crate::rts_unit::soul::RTSUnitSoulID;

#[derive(Component)]
pub struct TargetUnitDetection {
    target: Option<RTSUnitSoulID>,
    target_detection: Option<RTSUnitSoulID>,
}
impl Default for TargetUnitDetection {
    fn default() -> Self {
        return Self { 
            target: None,
            target_detection: None, 
        }
    }
}
impl TargetUnitDetection {
    pub fn new() -> Self {
        return Self {
            target: None,
            target_detection: None, 
        }
    }
}

impl TargetUnitDetection {
    fn set_target(&mut self, target: Option<RTSUnitSoulID>) {
        self.target = target;
    }

    pub fn target(&self) -> Option<RTSUnitSoulID> {
        return self.target
    }
}

impl SingleResultDetection for TargetUnitDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoulID>,
    ) {
        todo!()
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoulID> {
        todo!()
    }
}