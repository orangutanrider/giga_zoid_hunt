use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct RTSUnitSoulID(Entity);
impl Default for RTSUnitSoulID {
    fn default() -> Self {
        return Self::PLACEHOLDER
    }
}
impl RTSUnitSoulID {
    pub const PLACEHOLDER: Self = Self(Entity::PLACEHOLDER);

    pub fn new(entity: Entity) -> Self {
        return RTSUnitSoulID(entity)
    }

    pub fn entity(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
/// Attach to root entity, points to soul components' entity
/// (Attackable, Detectable)
pub struct RTSUnitSoulEntity(RTSUnitSoulID);
impl RTSUnitSoulEntity {
    pub fn new(soul_entity: Entity) -> Self {
        return Self(RTSUnitSoulID::new(soul_entity))
    }

    pub fn entity(&self) -> Entity {
        return self.0.0
    }
}