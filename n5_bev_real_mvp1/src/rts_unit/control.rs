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

#[derive(Component)]
/// Attach to root entity, points to control component's entity
/// (Selectable, Commandable)
pub struct RTSUnitControlEntity(Entity);
impl RTSUnitControlEntity {
    pub fn new(control_entity: Entity) -> Self {
        return Self(control_entity)
    }

    pub fn entity(&self) -> Entity {
        return self.0
    }
}