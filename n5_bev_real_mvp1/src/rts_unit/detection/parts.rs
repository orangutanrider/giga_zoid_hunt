pub use super::{
    single_result_types::{
        SingleResultDetection,
        arbitrary_unit::ArbitraryUnitDetection,
        closest_unit::ClosestUnitDetection,
        target_unit::{
            TargetUnitDetection,
            target_from_commandable::TargetFromCommandable,
        },
    },
    detectors::{
        AdditionalDetectorFilter,
        circle_intersections::CircleIntersectUnitDetector,
    }
};