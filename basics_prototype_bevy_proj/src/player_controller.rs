use bevy::prelude::*;
//use bevy::input::InputPlugin;
//use super::MainCamera;

pub struct InitializePlugin;
impl Plugin for InitializePlugin {
    fn build(&self, app: &mut App) {
        println!("");
        println!("Initializing player_controller");
        app
           .add_systems(PostStartup, spawn_test_unit);
    }
}

#[derive(Bundle)]
struct TestUnit{
    sprite_bundle: SpriteBundle,
}

fn spawn_test_unit(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(
        TestUnit{ 
            sprite_bundle: SpriteBundle { texture: asset_server.load("sprite\\basics\\64px_square.png"), ..default() }
        }
    );
}