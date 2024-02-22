pub mod r#move; // move.rs ('r#move' since 'move' is a keyword)
pub mod attack_target;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(r#move::InitializePlugin);
    }
}