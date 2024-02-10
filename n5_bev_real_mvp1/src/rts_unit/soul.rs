use bevy::prelude::*;
use super::rts_entity_impls;

#[derive(Clone, Copy)]
#[derive(Component)]
/// Attach to the control entity
/// An entity that is expected to be a soul sub-entity of an RTS unit, attached to the root of that unit in the transform
pub struct RTSUnitSoul(Entity);
rts_entity_impls!(RTSUnitSoul);

#[derive(Component)]
/// Attach to the root, points to the rts unit's soul entity
pub struct ToRTSUnitSoul(Entity);
rts_entity_impls!(ToRTSUnitSoul);