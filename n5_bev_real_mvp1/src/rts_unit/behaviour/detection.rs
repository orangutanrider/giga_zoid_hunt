pub mod attackable_circle_cast;

use crate::rts_unit::RTSUnitID;

pub trait ClosestUnitDetection {
    fn closest_unit_in_range(&self) -> Option<RTSUnitID>;
}