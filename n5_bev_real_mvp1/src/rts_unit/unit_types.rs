//pub mod enemy;
pub mod player;
//pub mod prelude;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(player::InitializePlugin);
    }
}

#[derive(Component)]
pub enum RtsTeam {
    Enemy,
    Player
}