use bevy::prelude::*;
use rts_unit_control::prelude::*;
use rts_unit_detectors::prelude::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

#[derive(Component, Default)]
/// Data transmission flag.
pub struct TargetAsCurrentInControl<S: RefSignature>{
    signature: PhantomData<S>
}

#[derive(Component, Default)]
/// Data-delivery, reference flag.
pub struct TargetIsLocal<S: RefSignature>{
    signature: PhantomData<S>
}

#[derive(Component, Default)]
/// Data-source, reference flag.
pub struct ControlIsReference<S: RefSignature>{
    signature: PhantomData<S>
}

/// target = TargetAsCurrentInControl + (TargetAsLocal, ControlAsReference)
pub fn target_from_control_via_reference_sys<S: RefSignature>(
    mut detector_q: Query<(&mut TDetectionTarget, &ToControl), (With<TargetIsLocal<S>>, With<ControlIsReference<S>>)>,
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