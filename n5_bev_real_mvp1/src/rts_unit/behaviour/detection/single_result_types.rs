pub mod closest_unit;
pub mod target_unit;
pub mod arbitrary_unit;

use crate::rts_unit::soul::RTSUnitSoulID;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(target_unit::InitializePlugin);
    }
}

pub trait SingleResultDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoulID>,
    );
    
    fn detection(
        &self
    ) -> Option<RTSUnitSoulID>;
}