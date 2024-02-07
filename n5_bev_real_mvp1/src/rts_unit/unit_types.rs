pub mod enemy;
pub mod player;
//pub mod prelude;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Group;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::InitializePlugin,
            enemy::InitializePlugin,
        ));
    }
}

#[derive(Component)]
pub enum RTSTeam{
    Player,
    Enemy,
}