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
pub struct ControlComponents{
    components_entity: Entity,
}
impl ControlComponents {
    pub fn new(components_entity: Entity) -> Self {
        return Self{
            components_entity,
        }
    }

    pub fn components_entity(&self) -> Entity {
        return self.components_entity
    }
}