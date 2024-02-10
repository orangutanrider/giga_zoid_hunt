pub mod enemy;
pub mod player;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::InitializePlugin,
            enemy::InitializePlugin,
        ));
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
pub enum RTSTeam{
    Player,
    Enemy,
}