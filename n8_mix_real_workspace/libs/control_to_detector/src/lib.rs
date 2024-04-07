use bevy::prelude::*;
use rts_unit_control::prelude::*;
use rts_unit_detectors::prelude::*;

use ref_caravan::*;
use ref_paths::*;

#[derive(Component)]
/// Data transmission flag.
pub struct TargetAsCurrentInControl;

#[derive(Component)]
/// Data-delivery, reference flag.
pub struct TargetIsLocal;

#[derive(Component)]
/// Data-source, reference flag.
pub struct ControlIsReference;

/// target = TargetAsCurrentInControl + (TargetAsLocal, ControlAsReference)
pub fn target_from_control_via_reference_sys(
    mut detector_q: Query<(&mut TDetectionTarget, &ToControl), (With<TargetIsLocal>, With<ControlIsReference>)>,
    control_q: Query<&CurrentTarget>
) {
    for (terminal, to_control) in detector_q.iter_mut() {
        target_from_control_via_reference(terminal, to_control, &control_q);
    }
}

fn target_from_control_via_reference(
    mut terminal: Mut<TDetectionTarget>,
    to_control: &ToControl,
    control_q: &Query<&CurrentTarget>
) {
    ref_caravan!(to_control::control_q(current_target););
    terminal.0 = current_target.read();
}