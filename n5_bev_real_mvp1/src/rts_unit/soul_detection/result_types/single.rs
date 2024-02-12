pub mod closest;
pub mod target;
pub mod arbitrary;

use crate::rts_unit::soul::RTSUnitSoul;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(target::InitializePlugin);
    }
}

pub trait SingleResultDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoul>,
    );
    
    fn detection(
        &self
    ) -> Option<RTSUnitSoul>;
}