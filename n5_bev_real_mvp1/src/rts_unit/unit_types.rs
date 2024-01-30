//pub mod enemy;
//pub mod player;
//pub mod prelude;

use bevy::prelude::*;

#[derive(Component)]
pub enum RtsTeam {
    Enemy,
    Player
}