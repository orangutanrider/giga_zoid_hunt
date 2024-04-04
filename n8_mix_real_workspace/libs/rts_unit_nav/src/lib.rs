//! Navigation = Order to mover vector

pub mod follow_attack_move;
pub mod follow_attack_target;
pub mod follow_pure_move;

use std::any::TypeId;
use bevy::prelude::*;

use ref_paths::*;

#[derive(Component)]
pub struct ToNav(Entity);
waymark!(ToNav);

// Type terminal

#[derive(Component)]
pub struct TNavType(pub TypeId);

// Data terminals

#[derive(Component)]
/// Waypoint Data terminal.
pub struct TNavPureMove(pub Vec2);

#[derive(Component)]
/// Waypoint Data terminal.
pub struct TNavAttackMove(pub Vec2);

#[derive(Component)]
/// Waypoint Data terminal.
pub struct TNavAttackTarget(pub Vec2);

#[macro_export]
macro_rules! c_validate_data_terminal { ($nav_type:ty, $type_terminal:ident) => {
    if $type_terminal.0 != TypeId::of::<$nav_type>() {
        continue;
    }
};}

// Output

#[derive(Component)]
pub struct NavVectorOutput(pub Vec2);