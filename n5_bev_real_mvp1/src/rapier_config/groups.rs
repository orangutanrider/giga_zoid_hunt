use bevy_rapier2d::prelude::Group;

// Player team groups
pub const P_SELECTABLE: Group = Group::GROUP_1; 
pub const P_ATTACKABLE: Group = Group::GROUP_2; 
pub const P_PRINCE: Group = Group::GROUP_3;

// Enemy team groups
pub const E_ATTACKABLE: Group = Group::GROUP_6;