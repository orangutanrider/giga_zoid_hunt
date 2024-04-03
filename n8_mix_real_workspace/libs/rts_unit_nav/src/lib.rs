//! Navigation = Order to mover vector

pub mod follow_attack_move;
pub mod follow_attack_target;
pub mod follow_pure_move;

use std::any::TypeId;

use bevy::prelude::*;
use rts_unit_control::prelude::*;


// Type terminal

#[derive(Component)]
pub struct TNavType(pub TypeId);

// Data terminals

#[derive(Component)]
pub struct TNavPureMove(pub PureMoveOrder);

#[derive(Component)]
pub struct TNavAttackMove(pub AttackMoveOrder);

#[derive(Component)]
pub struct TNavAttackTarget(pub AttackTargetOrder);

#[macro_export]
macro_rules! c_validate_data_terminal { ($data_type:ty, $type_terminal:ident) => {
    if $type_terminal.0 != TypeId::of::<$data_type>() {
        continue;
    }
};}

// Data transfer flag(s).
// Combine with reference flag.
// (What data?)

#[derive(Component)]
/// Data Transfer Flag.
/// Combine with reference flag.
pub struct NavAsCurrentOrderInControl;

// Reference flag(s)
// Combine with data transfer flag.
// (Where to get the data from.)

#[derive(Component)]
/// Reference Flag.
/// Combine with data transfer flag.
/// (Nav -> Root -> Control)
pub struct NavOrderFromControlViaRoot;

// Example reference flags
/* 

#[derive(Component)]
pub struct NavOrderFromLocal;

#[derive(Component)]
pub struct NavOrderFromDirect(Entity);
impl NavOrderFromDirect {...}

*/

// Output

#[derive(Component)]
pub struct NavVectorOutput(pub Vec2);