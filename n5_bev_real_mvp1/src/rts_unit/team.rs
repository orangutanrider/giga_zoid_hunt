pub mod enemy;
pub mod player;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerTeamUnitFlag;

#[derive(Component)]
pub struct EnemyTeamUnitFlag;