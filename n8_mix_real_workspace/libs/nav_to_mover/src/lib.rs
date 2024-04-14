use bevy::prelude::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

use rts_unit_nav::*;
use rts_unit_movers::*;

// Data transmission flags.
/* 
#[derive(Component)]
/// Data transmission flag.
pub struct MoveAsNav;
*/

#[derive(Component)]
/// Data transmission flag.
pub struct SwitchedMoveAsNav<S: RefSignature>{
    pub switch: bool,
    signature: PhantomData<S>
}
impl<S: RefSignature> Default for SwitchedMoveAsNav<S> {
    fn default() -> Self {
        Self { switch: false, signature: Default::default() }
    }
}
impl<S: RefSignature> SwitchedTransmissionFlag for SwitchedMoveAsNav<S> {
    fn set(&mut self, v: bool) {
        self.switch = v;
    }

    fn read(&self) -> bool {
        return self.switch;
    }
}

// Data-source, reference flags.
/* 
#[derive(Component)]
/// Data-source, reference flag.
pub struct NavIsLocal;
*/

#[derive(Component, Default)]
/// Data-source, reference flag.
pub struct NavIsReference<S: RefSignature>{
    signature: PhantomData<S>
}

// Data-delivery, reference flags.
/* 
#[derive(Component)]
/// Data-delivery, reference flag.
pub struct MoveIsLocal;
*/

#[derive(Component, Default)]
/// Data-delivery, reference flag.
pub struct MoveIsReference<S: RefSignature>{
    signature: PhantomData<S>
}

// Systems matrix
/// Move = MoveAsNav + (NavIsLocal + MoveIsLocal)
/* 
pub fn move_vector_from_nav_sys(
    mut q: Query<(&mut TMoveVector, &NavVectorOutput), (With<MoveAsNav>, With<NavIsLocal>, With<MoveIsLocal>)>,
) {
    for (mut mover, nav) in q.iter_mut() {
        mover.0 = nav.0;
    }
}
*/

/// Move = SwitchedMoveAsNav + (NavIsReference + MoveIsReference)
pub fn switched_reference_move_as_reference_nav_sys<S: RefSignature>(
    q: Query<(&ToNav, &ToMover, &SwitchedMoveAsNav<S>), (With<NavIsReference<S>>, With<MoveIsReference<S>>)>,
    nav_q: Query<&NavVectorOutput>,
    mut move_q: Query<&mut TMoveVector>,
) {
    for (to_nav, to_mover, switch) in q.iter() {
        if !switch.switch {
            continue
        }
        switched_reference_move_as_reference_nav(to_nav, to_mover, &nav_q, &mut move_q);
    }
}

pub fn switched_reference_move_as_reference_nav(
    to_nav: &ToNav,
    to_mover: &ToMover,
    nav_q: &Query<&NavVectorOutput>,
    move_q: &mut Query<&mut TMoveVector>,
) {
    ref_caravan!(
        to_nav::nav_q(nav_vector);
        to_mover::move_q(mut mover);
    );

    mover.0 = nav_vector.0;
}