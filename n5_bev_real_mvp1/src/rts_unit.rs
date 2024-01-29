pub mod control;
pub mod movement;
pub mod team;
pub mod unit_components;

use bevy::prelude::*;


pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit");
        app.add_plugins((
            movement::InitializePlugin,
            commandable::InitializePlugin,
        ));
    }
}

#[derive(Component)]
pub struct Unit {
    pub id: UnitID,
}

#[derive(Clone, Copy)]
pub struct UnitID(pub Entity);
impl UnitID {
    pub const PLACEHOLDER: UnitID = UnitID(Entity::PLACEHOLDER);
}