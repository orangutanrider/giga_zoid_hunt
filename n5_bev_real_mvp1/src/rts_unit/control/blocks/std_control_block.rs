use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::rts_unit::{
    *,
    block::IntegratedBlock,
    control::parts::*,
    control::*,
};

use super::STANDARD_SELECTABLE_SIZE;
use crate::rapier_config::collision_groups::P_CONTROL_CGROUP;

#[derive(Bundle)]
pub struct StdControlBlock {
    pub to_root: ToRoot,
    pub transform: TransformBundle,

    pub collider: Collider, // Selectable
    pub sensor: Sensor,
    pub c_group: CollisionGroups,

    pub commandable: Commandable,
    pub selectable: Selectable,
}

#[derive(Clone, Copy)]
pub struct Parameters {
    pub position: Vec2,
}

#[derive(Clone, Copy)]
pub struct EntityReferences {
    pub root: RTSUnitRoot,
}

impl IntegratedBlock<StdControlBlock, Parameters, EntityReferences> for StdControlBlock {
    fn new_bundle(params: Parameters, entity_references: EntityReferences) -> StdControlBlock {
        return Self {
            to_root: ToRoot::new(entity_references.root.entity()),
            transform: TransformBundle { 
                local: Transform { 
                    translation: params.position.extend(0.0), 
                    ..default()
                }, ..default() 
            },

            collider: Collider::cuboid(STANDARD_SELECTABLE_SIZE, STANDARD_SELECTABLE_SIZE),
            sensor: Sensor,
            c_group: P_CONTROL_CGROUP,

            commandable: Commandable::new(),
            selectable: Selectable::new(),
        }
    }

    fn spawn_complete(
        commands: &mut Commands,
        params: Parameters,
        entity_references: EntityReferences,
    ) -> Entity  {
        let parent = entity_references.root.entity();
        let entity = commands.spawn( Self::new_bundle(params, entity_references)).id();
        commands.entity(parent).insert(RootToControl(entity));
        commands.entity(entity).insert(RTSUnitControl::new(entity));
        commands.entity(parent).push_children(&[entity]);
        return entity
    }
}