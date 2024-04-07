pub mod attack_move;
pub mod pure_move;
pub mod attack_target;
pub mod idle;

use bevy::prelude::*;

#[derive(Component)]
/// Data-destination, reference flag.
pub struct NavIsLocal; 

#[derive(Component)]
/// Data-destination, reference flag.
pub struct ControlIsLocal; 