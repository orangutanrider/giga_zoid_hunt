use bevy_rapier2d::prelude::Group;

pub const RTS_PHYSICS: Group = Group::GROUP_18;
pub const RTS_DETECTION: Group = Group::GROUP_12;

// Player team groups
pub const P_SELECTABLE: Group = Group::GROUP_1; 
pub const P_ATTACKABLE: Group = Group::GROUP_2; 
pub const P_DETECTABLE: Group = Group::GROUP_4;
pub const P_PRINCE: Group = Group::GROUP_3;

pub const P_SOUL: Group = P_ATTACKABLE.union(P_DETECTABLE);

// Enemy team groups
pub const E_ATTACKABLE: Group = Group::GROUP_6;
pub const E_DETECTABLE: Group = Group::GROUP_5;

pub const E_SOUL: Group = E_ATTACKABLE.union(E_DETECTABLE);