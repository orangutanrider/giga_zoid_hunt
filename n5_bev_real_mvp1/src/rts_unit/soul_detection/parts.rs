pub use super::{
    result_types::single::{
        SingleResultDetection,
        arbitrary::TArbitrarySoulDetection,
        closest::TClosestSoulDetection,
        target::{
            TTargetSoulDetection,
            target_from_commandable::TargetFromCommandable,
        },
    },
    detectors::{
        AdditionalDetectorFilter,
        circle_intersections::CircleIntersectSoulDetector,
    }
};