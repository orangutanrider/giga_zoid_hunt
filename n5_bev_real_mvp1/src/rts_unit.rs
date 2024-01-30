pub mod control;
pub mod movement;
pub mod unit_types;
pub mod unit_components;
pub mod behaviour;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            control::InitializePlugin,
        ));
    }
}

#[derive(Component)]
pub struct RTSUnit {
    id: RTSUnitID,
}
impl Default for RTSUnit {
    fn default() -> Self {
        return Self::new(Entity::PLACEHOLDER)
    }
}
impl RTSUnit {
    pub fn new(entity: Entity) -> Self {
        return Self { id: RTSUnitID(entity) }
    }

    pub fn entity(&self) -> Entity {
        return self.id.0
    }
}

#[derive(Clone, Copy)]
pub struct RTSUnitID(Entity);
impl Default for RTSUnitID {
    fn default() -> Self {
        return Self::PLACEHOLDER
    }
}
impl RTSUnitID {
    pub const PLACEHOLDER: Self = Self(Entity::PLACEHOLDER);

    pub fn new(entity: Entity) -> Self {
        return RTSUnitID(entity)
    }
}