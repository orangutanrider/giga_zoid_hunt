mod proto_unit;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(proto_unit::InitializePlugin);
    }
}


#[derive(Component)]
struct EnemyTeam;

#[derive(Component)]
struct ProtoUnit;
