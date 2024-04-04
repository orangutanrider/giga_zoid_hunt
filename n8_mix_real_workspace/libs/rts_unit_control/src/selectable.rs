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

/// commands.insert(Selected);
pub fn select(
    mut commands: EntityCommands,
) {
    commands.insert(Selected);
}

/// for selected in q.iter() {
///     commands.entity(selected).remove::<Selected>();
/// }
pub fn un_select_all(
    commands: &mut Commands,
    q: &Query<Entity, With<Selected>>
) {
    for selected in q.iter() {
        commands.entity(selected).remove::<Selected>();
    }
}