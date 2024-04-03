use bevy::prelude::*;
use crate::*;

use ref_paths::*;
use ref_caravan::*;
use rts_waymarks::*;
use rts_unit_control::prelude::*;

#[derive(Component)]
pub struct DistillationForTarget(Option<Entity>);
impl DistillationColumn for DistillationForTarget {
    fn read_detection(&self) -> Option<Entity> {
        return self.0
    }

    fn distiller_set(&mut self, v: Option<Entity>) {
        self.0 = v;
    }
}

// Data terminal

#[derive(Component)]
pub struct TDetectionTarget(pub Option<Entity>);

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


#[derive(Component)]
/// Data Transfer Flag.
/// Combine with reference flag.
pub struct TargetAsCurrentInControl;

#[derive(Component)]
/// Reference Flag.
/// Combine with data transfer flag.
/// (Detector -> Root -> Control)
pub struct TargetFromControlViaRoot;

pub fn target_from_control_via_root_sys(
    mut detector_q: Query<(&mut TDetectionTarget, Entity), (With<TargetAsCurrentInControl>, With<TargetFromControlViaRoot>)>,
    root_q: Query<&ToRoot>,
    control_q: Query<&CurrentTarget>
) {
    for (terminal, entity) in detector_q.iter_mut() {
        target_from_control_via_root(terminal, entity, &root_q, &control_q)
    }
}

fn target_from_control_via_root(
    mut terminal: Mut<TDetectionTarget>,
    detector: Entity,
    root_q: &Query<&ToRoot>,
    control_q: &Query<&CurrentTarget>
) {
    ref_caravan!(@detector::root_q(to_root) -> to_root::control_q(current_target););
    terminal.0 = current_target.read();
}