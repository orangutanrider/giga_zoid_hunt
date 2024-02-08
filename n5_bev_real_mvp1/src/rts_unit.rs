pub mod control;
pub mod movement;
pub mod unit_types;
pub mod unit_components;
pub mod behaviour;
pub mod soul;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            control::InitializePlugin,
            unit_types::InitializePlugin,
            behaviour::InitializePlugin,
            movement::InitializePlugin,
        ));
    }
}

#[derive(Component)]
pub struct RTSUnit {
    id: RTSUnitID,
}
impl Default for RTSUnit {
    fn default() -> Self {
        return Self{id: RTSUnitID::PLACEHOLDER}
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

#[derive(Component)]
/// For entities attached to the root in the transform tree
pub struct ToRTSUnitRoot(Entity);
impl Default for ToRTSUnitRoot {
    fn default() -> Self {
        return Self(Entity::PLACEHOLDER)
    }
}
impl ToRTSUnitRoot {
    pub fn new(root: Entity) -> Self {
        return Self(root)
    }
    
    pub fn root(&self) -> RTSUnitID {
        return RTSUnitID(self.0)
    }
}

#[derive(Clone, Copy)]
/// The root entity
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