pub mod attack;
pub mod navigation;
pub mod detection;

use bevy::prelude::*;

#[derive(Component)]
/// Attach to root entity, points to behaviour component's entity
pub struct BehaviourComponents{
    components_entity: Entity,
}
impl BehaviourComponents {
    pub fn new(components_entity: Entity) -> Self {
        return Self{
            components_entity,
        }
    }

    pub fn components_entity(&self) -> Entity {
        return self.components_entity
    }
}