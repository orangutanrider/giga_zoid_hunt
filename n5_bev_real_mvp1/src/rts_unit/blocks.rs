use bevy::prelude::*;

/// A 'block' of functionality for a unit, in the form of a spawnable bundle
pub trait Block<B, Params, EntityReferences>
where B: Bundle {
    fn spawn_empty(
        commands: &mut Commands,
    ) -> Entity {
        return commands.spawn_empty().id()
    }

    fn spawn_complete_onto(
        commands: &mut Commands,
        parent: Entity,
        params: Params,
    ) -> Entity {
        let entity = commands.spawn( Self::new_bundle(params)).id();
        commands.entity(parent).push_children(&[entity]);
        return entity
    }

    fn new_complete_bundle(params: Params, entity_references: EntityReferences) -> B;
}

pub trait IntegratedBlockSpawn<B, Params, EntityReferences>
where B: Bundle {
    fn spawn_complete(
        commands: &mut Commands,
        params: Params,
        entity_references: EntityReferences,
    ) -> Entity ;
}