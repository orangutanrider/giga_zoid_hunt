pub mod closest_unit;
pub mod target_unit;
pub mod arbitrary_unit;

use crate::rts_unit::soul::RTSUnitSoulID;

pub trait SingleResultDetection {
    fn set_detection(
        &mut self,
        detection: Option<RTSUnitSoulID>,
    );
    
    fn detection(
        &self
    ) -> Option<RTSUnitSoulID>;
}