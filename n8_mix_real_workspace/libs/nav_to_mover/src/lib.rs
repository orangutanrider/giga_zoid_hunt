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
pub struct SwitchedMoveAsNav<Signature: RefSignature>{
    pub switch: bool,
    signature: PhantomData<Signature>
}

// Data-source, reference flags.
/* 
#[derive(Component)]
/// Data-source, reference flag.
pub struct NavIsLocal;
*/

#[derive(Component)]
/// Data-source, reference flag.
pub struct NavIsReference<Signature: RefSignature>{
    signature: PhantomData<Signature>
}

// Data-delivery, reference flags.
/* 
#[derive(Component)]
/// Data-delivery, reference flag.
pub struct MoveIsLocal;
*/

#[derive(Component)]
/// Data-delivery, reference flag.
pub struct MoveIsReference<Signature: RefSignature>{
    signature: PhantomData<Signature>
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
pub fn switched_reference_move_as_reference_nav_sys<Signature: RefSignature>(
    q: Query<(&ToNav, &ToMover, &SwitchedMoveAsNav<Signature>), (With<NavIsReference<Signature>>, With<MoveIsReference<Signature>>)>,
    nav_q: Query<&NavVectorOutput>,
    mut move_q: Query<&mut TMoveVector>,
) {
    for (to_nav, to_mover, switch) in q.iter() {
        if !switch.switch {
            continue
        }

        ref_caravan!(
            to_nav::nav_q(nav_vector);
            to_mover::move_q(mut mover);
        );

        mover.0 = nav_vector.0;
    }
}