pub mod parts;

use bevy::prelude::*;

use crate::rts_unit::*;
use crate::entity_ref_impls;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct RTSUnitSoul(Entity);
entity_ref_impls!(RTSUnitSoul, SelfEntity);

#[derive(Component)]
pub struct RootToSoul(Entity);
entity_ref_impls!(RootToSoul, ChildEntity);

#[derive(Component)]
pub struct ChildToSoul(Entity);
entity_ref_impls!(ChildToSoul, ParentEntity);