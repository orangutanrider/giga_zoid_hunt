use bevy_rapier2d::prelude::Group;

// The groups membership indicates what groups the collider is part of (one bit per group).
// The groups filter indicates what groups the collider can interact with (one bit per group).

// Player team groups
pub const P_SELECTABLE: Group = Group::GROUP_1; 
pub const P_ATTACKABLE: Group = Group::GROUP_2; 
pub const P_PRINCE: Group = Group::GROUP_3;

// Enemy team groups
pub const E_ATTACKABLE: Group = Group::GROUP_6;