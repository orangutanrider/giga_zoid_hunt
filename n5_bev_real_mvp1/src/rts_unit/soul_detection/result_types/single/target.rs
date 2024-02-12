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
pub struct TargetSoulDetection {
    target: Option<RTSUnitSoul>,
    target_detection: Option<RTSUnitSoul>,
}
impl Default for TargetSoulDetection {
    fn default() -> Self {
        return Self { 
            target: None,
            target_detection: None, 
        }
    }
}
impl TargetSoulDetection {
    pub fn new() -> Self {
        return Self {
            target: None,
            target_detection: None, 
        }
    }
}

impl TargetSoulDetection {
    fn set_target(&mut self, target: Option<RTSUnitSoul>) {
        self.target = target;
    }

    pub fn target(&self) -> Option<RTSUnitSoul> {
        return self.target
    }
}

impl SingleResultDetection for TargetSoulDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoul>,
    ) {
        self.target_detection = detection;
    }

    fn detection(
        &self
    ) -> Option<RTSUnitSoul> {
        return self.target_detection
    }
}