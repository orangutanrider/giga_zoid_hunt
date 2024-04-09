// Could be re-created as a seperate lib.

use super::*;

#[derive(Component)]
pub(crate) struct BangToSwitch<S: RefSignature> {
    signature: PhantomData<S>,
}

pub(crate) fn bang_to_switch_sys<Transmission: SwitchedTransmissionFlag, Flag: Component, S: RefSignature>(
    mut q: Query<(&Bang, &mut Transmission), (Changed<Bang>, With<Flag>)>
) {
    for (bang, mut switch) in q.iter_mut() {
        switch.set(bang.is_active());
    }
}

// Hmm...
// The readability of these, something should be improved there.

// Bang to switch bundles
#[derive(Bundle)]
pub(crate) struct BangToSwitchedMoveAsNav {
    pub flag: BangToSwitch<BangToSwitchedMoveAsNav>,
}
impl Plugin for BangToSwitchedMoveAsNav {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            bang_to_switch_sys::<SwitchedMoveAsNav<BangToSwitchedMoveAsNav>, BangToSwitch<BangToSwitchedMoveAsNav>, BangToSwitchedMoveAsNav>,
        ));
    }
}
ref_signature!(BangToSwitchedMoveAsNav);

// Bang to switch bundles
#[derive(Bundle)]
pub(crate) struct BangToSwitchedControlAsNav {
    pub flag: BangToSwitch<BangToSwitchedControlAsNav>,
}
impl Plugin for BangToSwitchedControlAsNav {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            bang_to_switch_sys::<SwitchedMoveAsNav<BangToSwitchedControlAsNav>, BangToSwitch<BangToSwitchedControlAsNav>, BangToSwitchedControlAsNav>,
        ));
    }
}
ref_signature!(BangToSwitchedControlAsNav);