// Could be re-created as a seperate lib.

use super::*;

#[derive(Component, Default)]
pub(crate) struct BangToSwitch<S: RefSignature> {
    signature: PhantomData<S>,
}

pub(crate) fn bang_to_switch_sys<T, F, S>(
    mut q: Query<(&Bang, &mut T), (Changed<Bang>, With<F>)>
) where
    T: SwitchedTransmissionFlag,
    F: Component, // Bang to switch flag component
    S: RefSignature,
{
    for (bang, mut switch) in q.iter_mut() {
        switch.set(bang.is_active());
    }
}

// lazy implementation.
#[derive(Component, Default)]
pub struct RefdMoverIsZeroWhenBang;

pub fn refd_mover_is_zero_when_bang_sys(
    q: Query<(&Bang, &ToMover), With<RefdMoverIsZeroWhenBang>>,
    mut mover_q: Query<&mut TMoveVector>,
) {
    for (bang, to_mover) in q.iter() {
        if !bang.is_active() {
            continue;
        }

        refd_mover_is_zero_when_bang(to_mover, &mut mover_q);
    }
}

fn refd_mover_is_zero_when_bang(
    to_mover: &ToMover,
    mover_q: &mut Query<&mut TMoveVector>,
) {
    ref_caravan!(to_mover::mover_q(mut mover););
    mover.0 = Vec2::ZERO;
}


// Hmm...
// The readability of these, something should be improved there.
// Yeah, whatever is going on here, it's not great.

/* 
// Bang to switch bundles
#[derive(Bundle, Default)]
pub(crate) struct BangToSwitchedMoveAsNav {
    pub flag: BangToSwitch<BangToSwitchedMoveAsNav>,
}
pub struct BangToSwitchedMoveAsNavPlugin;
impl Plugin for BangToSwitchedMoveAsNavPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            bang_to_switch_sys::<SwitchedMoveAsNav<BangToSwitchedMoveAsNav>, BangToSwitch<BangToSwitchedMoveAsNav>, BangToSwitchedMoveAsNav>,
        ));
    }
}
ref_signature!(BangToSwitchedMoveAsNav);

#[derive(Bundle, Default)]
pub(crate) struct BangToSwitchedControlAsNav {
    pub flag: BangToSwitch<BangToSwitchedControlAsNav>,
}
pub struct BangToSwitchedControlAsNavPlugin;
impl Plugin for BangToSwitchedControlAsNavPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            bang_to_switch_sys::<SwitchedMoveAsNav<BangToSwitchedControlAsNav>, BangToSwitch<BangToSwitchedControlAsNav>, BangToSwitchedControlAsNav>,
        ));
    }
}
ref_signature!(BangToSwitchedControlAsNav);
*/