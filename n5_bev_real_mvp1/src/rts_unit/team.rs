pub mod enemy;
pub mod player;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerTeamRtsUnit;

#[derive(Component)]
pub struct EnemyTeamRtsUnit;