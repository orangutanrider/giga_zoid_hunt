use bevy::prelude::*;

#[derive(Component)]
pub struct ToAttackTargetDetection(Entity);
impl Default for ToAttackTargetDetection {
    fn default() -> Self {
        return Self(Entity::PLACEHOLDER)
    }
}
impl ToAttackTargetDetection {
    pub fn new(entity: Entity) -> Self {
        return Self(entity)
    }
    
    pub fn entity(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
pub struct ToAttackArbitraryDetection(Entity);
impl Default for ToAttackArbitraryDetection {
    fn default() -> Self {
        return Self(Entity::PLACEHOLDER)
    }
}
impl ToAttackArbitraryDetection {
    pub fn new(entity: Entity) -> Self {
        return Self(entity)
    }
    
    pub fn entity(&self) -> Entity {
        return self.0
    }
}