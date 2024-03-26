use bevy::{
    ecs::system::{
        EntityCommands, 
        SystemParam
    }, 
    prelude::*
};

#[derive(Component)]
/// Static flag, dennoting if a unit can be selected.
pub struct Selectable;
impl Default for Selectable {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Component)]
/// Dynamic flag that gets added and removed from units.
pub struct Selected;

#[derive(SystemParam)]
pub struct SelectCommands<'w, 's>(Commands<'w, 's>);
impl<'w, 's> SelectCommands<'w, 's> {
    pub fn select(
        &mut self, 
        control: Entity
    ) {
        let mut commands = self.0.entity(control);
        commands.insert(Selected);
    }
}

/// commands.insert(Selected);
pub fn select(
    mut commands: EntityCommands,
) {
    commands.insert(Selected);
}