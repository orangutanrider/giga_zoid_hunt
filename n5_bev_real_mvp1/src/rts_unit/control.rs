pub mod commandable;
pub mod selectable;
pub mod prelude;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            selectable::InitializePlugin,
        ));
    }
}

#[derive(Clone, Copy)]
pub struct RTSUnitControlID(Entity);
impl Default for RTSUnitControlID {
    fn default() -> Self {
        return Self::PLACEHOLDER
    }
}
impl RTSUnitControlID {
    pub const PLACEHOLDER: Self = Self(Entity::PLACEHOLDER);

    pub fn new(entity: Entity) -> Self {
        return Self(entity)
    }

    pub fn entity(&self) -> Entity {
        return self.0
    }
}

#[derive(Component)]
/// Attach to root entity, points to control components' entity
/// (Selectable, Commandable)
pub struct RTSUnitControlEntity(RTSUnitControlID);
impl RTSUnitControlEntity {
    pub fn new(control_entity: Entity) -> Self {
        return Self(RTSUnitControlID::new(control_entity))
    }

    pub fn entity(&self) -> Entity {
        return self.0.0
    }
}