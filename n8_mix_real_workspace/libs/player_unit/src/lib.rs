pub(crate) mod chase_behav; pub(crate) use self::chase_behav::*;
pub(crate) mod common; pub(crate) use self::common::*;
pub(crate) mod move_behav; pub(crate) use self::move_behav::*;
pub(crate) mod state_to_root; pub(crate) use self::state_to_root::*;
pub(crate) mod attack_behav; pub(crate) use self::attack_behav::*;

pub(crate) use std::any::*;
pub(crate) use std::marker::*;
pub(crate) use bevy::prelude::*;

pub(crate) use ref_caravan::*;
pub(crate) use ref_paths::*;
pub(crate) use ref_marks::*;

pub(crate) use behaviour_tree::{prelude::*, state::State as TreeState};

pub(crate) use rts_unit_control::prelude::*;
pub(crate) use rts_unit_detectors::prelude::*;
pub(crate) use rts_unit_nav::*;

pub(crate) use nav_to_mover::*;

// Note:
// There are reference definitions in this that could be upgraded to be more flexible.

// ================================
// Unit Structure

#[derive(Component)]
struct Hub;
#[derive(Bundle)]
struct BHub {
    
}

#[derive(Component)]
struct Root;
#[derive(Bundle)]
struct BRoot {
    
}

#[derive(Component)]
struct AggroDetection;
#[derive(Bundle)]
struct BAggroDetection {
    
}

#[derive(Component)]
struct AttackDetection;
#[derive(Bundle)]
struct BAttackDetection {
    
}

#[derive(Component)]
struct Attacking;
#[derive(Bundle)]
struct BAttacking {
    
}