use bevy_rapier2d::prelude::Group;

pub const RTS_PHYSICS: Group = Group::GROUP_1;

// Player
pub const P_HITTABLE: Group = Group::GROUP_2;
pub const P_DETECTABLE: Group = Group::GROUP_3;
pub const SELECTABLE: Group = Group::GROUP_4;

// Enemy
pub const E_HITTABLE: Group = Group::GROUP_5;
pub const E_DETECTABLE: Group = Group::GROUP_6;
