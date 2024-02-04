use bevy::prelude::*;

use super::SingleResultDetection;
use crate::rts_unit::RTSUnitID;

#[derive(Component)]
pub struct TargetUnitDetection {
    target: Option<RTSUnitID>,
    target_detection: Option<RTSUnitID>,
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
    pub fn set_target(&mut self, target: Option<RTSUnitID>) {
        self.target = target;
    }

    pub fn target(&self) -> Option<RTSUnitID> {
        return self.target
    }
}

impl SingleResultDetection for TargetUnitDetection {
    fn set_detection(
        &self,
        detection: Option<RTSUnitID>,
    ) {
        todo!()
    }

    fn detection(
        &self
    ) -> Option<RTSUnitID> {
        todo!()
    }
}