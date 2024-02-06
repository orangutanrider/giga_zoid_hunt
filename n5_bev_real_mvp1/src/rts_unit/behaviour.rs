pub mod attack;
pub mod navigation;
pub mod detection;
pub mod order_processing;

use bevy::prelude::*;

#[derive(Component)]
/// Attach to root entity, points to behaviour components' entity
pub struct RTSUnitBehaviourEntity(Entity);
impl Default for RTSUnitBehaviourEntity {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
impl RTSUnitBehaviourEntity {
    pub fn new(behaviour_entity: Entity) -> Self {
        return Self(behaviour_entity)
    }

    pub fn entity(&self) -> Entity {
        return self.0
    }
}