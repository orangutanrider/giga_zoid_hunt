pub mod closest_unit;
pub mod target_unit;

use crate::rts_unit::RTSUnitID;

pub trait SingleResultDetection {
    fn set_detection(
        &self,
        detection: Option<RTSUnitID>,
    );
    
    fn detection(
        &self
    ) -> Option<RTSUnitID>;
}