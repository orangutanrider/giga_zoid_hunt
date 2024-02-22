pub mod target_from_commandable;

use bevy::prelude::*;

use crate::rts_unit::{
    *,
    soul::RTSUnitSoul,
};
use super::SingleResultDetection;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(
            target_from_commandable::InitializePlugin
        );
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct TargetDetector(Entity);
entity_ref_impls!(TargetDetector, SelfEntity);

#[derive(Component)]
pub struct RootToTargetDetector(Entity);
entity_ref_impls!(RootToTargetDetector, ChildEntity);

#[derive(Component)]
pub struct ChildToTargetDetector(Entity);
entity_ref_impls!(ChildToTargetDetector, ParentEntity);

#[derive(Component)]
/// Target RTSUnitSoul Detection Terminal
pub struct TTargetSoulDetection {
    target: Option<RTSUnitSoul>,
    detection: Option<RTSUnitSoul>,
}
impl Default for TTargetSoulDetection {
    fn default() -> Self {
        return Self { 
            target: None,
            detection: None, 
        }
    }
}
impl TTargetSoulDetection {
    pub fn new() -> Self {
        return Self {
            target: None,
            detection: None, 
        }
    }
}

impl TTargetSoulDetection {
    fn set_target(&mut self, target: Option<RTSUnitSoul>) {
        self.target = target;
    }

    pub fn target(&self) -> Option<RTSUnitSoul> {
        return self.target
    }
}

impl SingleResultDetection for TTargetSoulDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoul>,
    ) {
        self.detection = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoul> {
        return self.detection
    }
}