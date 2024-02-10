mod attack;
mod navigation;
mod get;
mod order_processing;

pub mod blocks;
pub mod parts;

use bevy::prelude::*;
use super::rts_entity_impls;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            navigation::InitializePlugin,
            detection::InitializePlugin,
            order_processing::InitializePlugin,
        ));
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
/// Attach to the control entity
/// An entity that is expected to be a behaviour sub-entity of an RTS unit, attached to the root of that unit in the transform
pub struct RTSUnitBehaviour(Entity);
rts_entity_impls!(RTSUnitBehaviour);

#[derive(Component)]
/// Attach to the root, points to the rts unit's behaviour entity
pub struct ToRTSUnitBehaviour(Entity);
rts_entity_impls!(ToRTSUnitBehaviour);