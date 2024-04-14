pub mod attack_move; pub use attack_move::*;
pub mod pure_move; pub use pure_move::*;
pub mod attack_target; pub use attack_target::*;
pub mod idle; pub use idle::*;

use bevy::prelude::*;

use std::marker::*;
use ref_marks::*;

/* 
#[derive(Component)]
/// Data-destination, reference flag.
pub struct NavIsLocal<S: RefSignature>{
    signature: PhantomData<S>
} 

#[derive(Component)]
/// Data-source, reference flag.
pub struct ControlIsLocal<S: RefSignature>{
    signature: PhantomData<S>
}
*/

#[derive(Component, Default)]
/// Data-destination, reference flag.
pub struct NavIsReference<S: RefSignature>{
    signature: PhantomData<S>
}

#[derive(Component, Default)]
/// Data-source, reference flag.
pub struct ControlIsReference<S: RefSignature>{
    signature: PhantomData<S>
}
