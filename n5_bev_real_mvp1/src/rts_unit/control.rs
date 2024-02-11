mod commandable;
mod selectable;

pub mod blocks;
pub mod parts;

use bevy::prelude::*;

use crate::rts_unit::*;
use crate::entity_ref_impls;

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
pub struct RTSUnitControl(Entity);
entity_ref_impls!(RTSUnitControl, SelfEntity);

#[derive(Component)]
pub struct RootToControl(Entity);
entity_ref_impls!(RootToControl, ChildEntity);

#[derive(Component)]
pub struct ChildToControl(Entity);
entity_ref_impls!(ChildToControl, ParentEntity);