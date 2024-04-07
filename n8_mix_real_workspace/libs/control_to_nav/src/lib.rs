pub mod attack_move;
pub mod pure_move;
pub mod attack_target;
pub mod idle;

use bevy::prelude::*;

use std::marker::*;
use ref_marks::*;
use ref_caravan::*;
use ref_paths::*;

#[derive(Component)]
/// Data-destination, reference flag.
pub struct NavIsLocal<S: RefSignature>{
    signature: PhantomData<S>
} 

#[derive(Component)]
/// Data-destination, reference flag.
pub struct ControlIsLocal<S: RefSignature>{
    signature: PhantomData<S>
}