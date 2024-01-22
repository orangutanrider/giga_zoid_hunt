mod proto_unit;

use bevy::prelude::*;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("Initializing unit::player_types::proto_unit");
        app
        .add_plugins(proto_unit::InitializePlugin);
    }
}


#[derive(Component)]
struct PlayerTeam;

#[derive(Component)]
struct ProtoUnit;
