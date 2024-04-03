use bevy::prelude::*;

use ref_caravan::ref_caravan;
use ref_paths::*;

use rts_unit_nav::{NavVectorOutput, ToNav};
use rts_unit_movers::TMoveVector;

#[derive(Component)]
/// Data transfer flag.
pub struct MoveVectorAsNavOutput;

#[derive(Component)]
/// Reference flag.
/// From navigation is local so it is expected that a ToNav is local (Expected usage for root entities).
pub struct MoveVectorFromNav;

pub fn move_vector_from_nav_sys(
    mut q: Query<(&mut TMoveVector, &ToNav), (With<MoveVectorAsNavOutput>, With<MoveVectorFromNav>)>,
    nav_q: Query<&NavVectorOutput>,
) {
    for (terminal, to_nav) in q.iter_mut() {
        move_vector_from_nav(terminal, to_nav, &nav_q);
    }
}

fn move_vector_from_nav(
    mut terminal: Mut<TMoveVector>,
    to_nav: &ToNav,
    nav_q: &Query<&NavVectorOutput>,
) {
    ref_caravan!(to_nav::nav_q(nav_vector));
    terminal.0 = nav_vector.0;
}