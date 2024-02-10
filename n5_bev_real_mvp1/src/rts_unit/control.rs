mod commandable;
mod selectable;

pub mod blocks;
pub mod parts;

use bevy::prelude::*;
use super::rts_entity_impls;


pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            selectable::InitializePlugin,
        ));
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
/// Attach to the control entity
/// An entity that is expected to be a control sub-entity of an RTS unit, attached to the root of that unit in the transform
pub struct RTSUnitControl(Entity);
rts_entity_impls!(RTSUnitControl);

#[derive(Component)]
/// Attach to the root, points to the rts unit's control entity
pub struct ToRTSUnitControl(Entity);
rts_entity_impls!(ToRTSUnitControl);