use bevy::prelude::*;
use crate::*;

#[derive(Component)]
pub struct DistillationForTarget(Option<Entity>);
impl Default for DistillationForTarget {
    fn default() -> Self {
        Self(None)
    }
}
impl DistillationColumn for DistillationForTarget {
    fn read_detection(&self) -> Option<Entity> {
        return self.0
    }

    fn distiller_set(&mut self, v: Option<Entity>) {
        self.0 = v;
    }
}

#[derive(Component)]
/// Data terminal. Input.
pub struct TDetectionTarget(pub Option<Entity>);
impl Default for TDetectionTarget {
    fn default() -> Self {
        Self(None)
    }
}

pub fn target_detection_distillation_sys(
    mut q: Query<(&mut DistillationForTarget, &TDetectionTarget, &TIntersectionsAggregate)>,
) {
    for (column, target, aggregate) in q.iter_mut() {
        let Some(target) = target.0 else {
            continue; // no target to look for
        };

        let mut target_found: Option<Entity> = None;
        let distillation_logic = |agg| -> Option<Entity> {
            if agg == target {
                target_found = Some(agg);
            }
            return target_found
        };

        distill(column, aggregate, distillation_logic);
    }
}